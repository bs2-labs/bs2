pub mod execution_table;
pub mod main_circuit;
pub mod memory_table;

use crate::main_circuit::MainCircuit;
use entry_builder::builder::EntryBuilder;
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
use runtime::trace::Step;
use runtime::trace::Trace;
use std::fs::File;
use std::io::BufReader;

pub fn prove(trace_path: &str) {
    let mut entry_builder = EntryBuilder::new();
    let steps = get_trace_from_file(trace_path);
    let trace = Trace {
        cycles: 0,
        return_value: 0,
        steps,
    };
    entry_builder.build(&trace).expect("build entry failed");
    // dbg!(entry_builder.entries.get_op_steps());

    let degree = 4u32;

    let circuit = MainCircuit::<Fr>::init(entry_builder.entries);

    let mut rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);

    let general_params = ParamsKZG::<Bn256>::setup(degree, &mut rng);
    let verifier_params: ParamsVerifierKZG<Bn256> = general_params.verifier_params().clone();
    // Initialize the proving key
    let vk = keygen_vk(&general_params, &circuit).expect("keygen_vk should not fail");
    let pk = keygen_pk(&general_params, vk, &circuit).expect("keygen_pk should not fail");
    // Create a proof
    let mut transcript = Blake2bWrite::<_, G1Affine, Challenge255<_>>::init(vec![]);
    let now = std::time::Instant::now();
    println!("Begin create proof");
    create_proof::<
        KZGCommitmentScheme<Bn256>,
        ProverSHPLONK<'_, Bn256>,
        Challenge255<G1Affine>,
        XorShiftRng,
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

fn get_trace_from_file(path: &str) -> Vec<Step> {
    let file = File::open(path).expect("open file");
    let reader = BufReader::new(file);
    let trace = serde_json::from_reader(reader).expect("read json");
    trace
}
