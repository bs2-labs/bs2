use std::{fs::File, io::BufReader};

use crate::exec::run::exec_run;
use clap::{command, Args, Parser, Subcommand};
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
    let verifier_params: ParamsVerifierKZG<Bn256> = general_params.verifier_params().clone();
    // Initialize the proving key
    let vk = keygen_vk(&general_params, &circuit).expect("keygen_vk should not fail");
    let pk = keygen_pk(&general_params, vk, &circuit).expect("keygen_pk should not fail");
    // Create a proof
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

    // Begin verify proof
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
