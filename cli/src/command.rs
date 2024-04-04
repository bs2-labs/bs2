use std::fmt::write;
use std::io::Write;
use std::{fs::File, io::BufReader};

use crate::exec::run::exec_run;
use clap::{command, Args, Parser, Subcommand};
use halo2_proofs::halo2curves::pairing::Engine;
use halo2_proofs::helpers::SerdeCurveAffine;
use halo2_proofs::plonk::VerifyingKey;
use halo2_proofs::poly::commitment::Params;
use halo2_proofs::{io, SerdeFormat};
use runtime::trace::Step;

use circuits::main_circuit::MainCircuit;
use entry_builder::builder::EntryBuilder;
use halo2_proofs::dev::MockProver;
use halo2_proofs::{
    halo2curves::bn256::{Bn256, Fr, G1Affine},
    plonk::{create_proof, keygen_pk, keygen_vk, verify_proof},
    poly::{
        commitment::ParamsProver,
        kzg::{
            commitment::{KZGCommitmentScheme, ParamsKZG, ParamsVerifierKZG},
            multiopen::{ProverSHPLONK, VerifierSHPLONK},
            strategy::SingleStrategy,
        },
    },
    transcript::{
        Blake2bRead, Blake2bWrite, Challenge255, TranscriptReadBuffer, TranscriptWriterBuffer,
    },
};
use rand_core::SeedableRng;
use rand_xorshift::XorShiftRng;
use runtime::trace::Trace;

pub const SHRINK_K: u32 = 1;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Run(RunArgs),
    Prove(RunArgs),
}

#[derive(Args)]
pub struct RunArgs {
    #[arg(short, long)]
    pub trace: Option<String>,
    // #[arg(short, long)]
    // pub bytecode: Option<String>,
    // #[arg(short, long)]
    // pub hardcode: Option<String>,
    // #[arg(short, long)]
    // pub file: Option<String>,
    // // #[arg(short, long)]
    // // pub dry_run: bool,
}

pub fn read_verifier_params<E: Engine, R: io::Read>(
    reader: &mut R,
) -> io::Result<ParamsVerifierKZG<E>>
where
    E::G1Affine: SerdeCurveAffine,
    E::G2Affine: SerdeCurveAffine,
{
    let shrink_k = SHRINK_K;
    let mut k = [0u8; 4];
    reader.read_exact(&mut k[..])?;
    let k = u32::from_le_bytes(k);
    let n = 1 << k;
    let shrink_n = 1 << shrink_k;

    let format = SerdeFormat::RawBytes;

    let g = (0..shrink_n)
        .map(|_| E::G1Affine::read(reader, format))
        .collect::<Result<Vec<_>, _>>()?;
    let g_lagrange = (0..shrink_n)
        .map(|_| E::G1Affine::read(reader, format))
        .collect::<Result<Vec<_>, _>>()?;

    let g2 = E::G2Affine::read(reader, format)?;
    let s_g2 = E::G2Affine::read(reader, format)?;

    Ok(ParamsVerifierKZG {
        k,
        n: n as u64,
        g,
        g_lagrange,
        g2,
        s_g2,
    })
}

pub fn prove(steps: Vec<Step>, rng: &mut XorShiftRng) {
    let mut entry_builder = EntryBuilder::new();
    let trace = Trace {
        cycles: 0,
        return_value: 0,
        steps,
    };
    entry_builder.build(&trace).expect("build entry failed");

    let degree = 14u32;

    let circuit = MainCircuit::<Fr>::init(entry_builder.entries);

    // let prover = MockProver::<Fr>::run(degree, &circuit, vec![]).unwrap();
    // let verify_result = prover.verify();

    // dbg!(verify_result);

    // return;

    // -----

    let general_params = ParamsKZG::<Bn256>::unsafe_setup(degree);
    let mut verifier_params: ParamsVerifierKZG<Bn256> = general_params.verifier_params().clone();
    verifier_params.shrink(SHRINK_K);
    let mut verifier_params_buf = alloc::vec![];
    verifier_params
        .write(&mut verifier_params_buf)
        .expect("write");
    println!("verifier parameters length : {}", verifier_params_buf.len());
    let mut file = File::create("params.hex").expect("open file");
    let vk_bytes_hex = hex::encode(&verifier_params_buf);
    file.write(&vk_bytes_hex.as_bytes()).expect("write vk hex");

    // check verification and serialization
    let verifier_params: ParamsVerifierKZG<Bn256> =
        read_verifier_params(&mut &verifier_params_buf[..verifier_params_buf.len()]).unwrap();

    // Initialize the proving key
    let vk = keygen_vk(&general_params, &circuit).expect("keygen_vk should not fail");
    let mut vk_bytes = vec![0u8; 1000];

    vk.write(&mut vk_bytes.as_mut_slice(), SerdeFormat::RawBytes)
        .expect("write vk");
    let mut file = File::create("vk.hex").expect("open file");
    let vk_bytes_hex = hex::encode(&vk_bytes);
    file.write(&vk_bytes_hex.as_bytes()).expect("write vk hex");

    println!("{}", hex::encode(&vk_bytes));

    VerifyingKey::<G1Affine>::read::<&[u8], MainCircuit<Fr>>(
        &mut vk_bytes.as_slice(),
        halo2_proofs::SerdeFormat::RawBytes,
    )
    .expect("Read vk");

    let pk = keygen_pk(&general_params, vk, &circuit).expect("keygen_pk should not fail");
    // Create a proof
    let now = std::time::Instant::now();
    println!("Begin create proof");

    let mut transcript = Blake2bWrite::<_, G1Affine, Challenge255<_>>::init(vec![]);
    create_proof::<
        KZGCommitmentScheme<Bn256>,
        ProverSHPLONK<'_, Bn256>,
        Challenge255<G1Affine>,
        &mut XorShiftRng,
        Blake2bWrite<Vec<u8>, G1Affine, Challenge255<G1Affine>>,
        MainCircuit<Fr>,
    >(
        &general_params,
        &pk,
        &[circuit],
        &[&[]],
        rng,
        &mut transcript,
    )
    .expect("proof generation should not fail");
    let proof = transcript.finalize();
    println!("Proof created, elapsed {:?}", now.elapsed());
    println!("{}", hex::encode(&proof));
    let mut file = File::create("proof.hex").expect("open file");
    let proof_hex = hex::encode(&proof);
    file.write(&proof_hex.as_bytes()).expect("write vk hex");

    // Begin verify proof
    let now = std::time::Instant::now();
    let mut verifier_transcript = Blake2bRead::<_, G1Affine, Challenge255<_>>::init(&proof[..]);
    let strategy = SingleStrategy::new(&general_params);

    verify_proof::<
        KZGCommitmentScheme<Bn256>,
        VerifierSHPLONK<'_, Bn256>,
        Challenge255<G1Affine>,
        Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
        SingleStrategy<'_, Bn256>,
    >(
        &verifier_params,
        pk.get_vk(),
        strategy,
        &[&[]],
        &mut verifier_transcript,
    )
    .expect("failed to verify circuit");
    println!("Verify proof, elapsed {:?}", now.elapsed());
}

pub fn match_operation(cli: &Cli) {
    match &cli.command {
        Commands::Run(_args) => {
            exec_run();
        }
        Commands::Prove(args) => {
            println!("create proof");
            let trace = args.trace.as_deref();
            let steps = get_trace_from_file(trace.unwrap());
            let mut rng = XorShiftRng::from_seed([
                0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06,
                0xbc, 0xe5,
            ]);

            prove(steps, &mut rng);
        }
    }
}

fn get_trace_from_file(path: &str) -> Vec<Step> {
    let file = File::open(path).expect("open file");
    let reader = BufReader::new(file);
    let trace = serde_json::from_reader(reader).expect("read json");
    trace
}
