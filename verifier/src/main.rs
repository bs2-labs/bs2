#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

#[cfg(test)]
extern crate alloc;


#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();

use alloc::{format, vec::Vec};
use circuits::main_circuit::MainCircuit;

use ckb_std::{
    ckb_constants::Source,
    syscalls::{debug, load_witness},
};
use halo2_gadgets::halo2curves::bn256::{Bn256, Fr, G1Affine};

use halo2_proofs::{
    helpers::SerdeCurveAffine,
    plonk::{verify_proof, VerifyingKey},
    poly::kzg::{
        commitment::{KZGCommitmentScheme, ParamsVerifierKZG},
        multiopen::VerifierSHPLONK,
        strategy::SingleStrategy,
    },
    transcript::{Blake2bRead, Challenge255, TranscriptReadBuffer},
    SerdeFormat,
};
use halo2curves::{io, pairing::Engine};

pub fn read_verifier_params<E: Engine, R: io::Read>(
    reader: &mut R,
) -> io::Result<ParamsVerifierKZG<E>>
where
    E::G1Affine: SerdeCurveAffine,
    E::G2Affine: SerdeCurveAffine,
{
    const SHRINK_K: u32 = 1;
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

pub fn program_entry() -> i8 {
    let mut params_buffer = [0u8; 1024];
    let params_len = match load_witness(&mut params_buffer, 0, 0, Source::Input) {
        Ok(l) => {
            debug(format!("Loading params length: {:?}", l));
            l
        }
        Err(e) => {
            debug(format!("Loading params error {:?}", e));
            return -1;
        }
    };
    let mut vk_buffer = [0u8; 4096];
    let vk_len = match load_witness(&mut vk_buffer, 0, 1, Source::Input) {
        Ok(l) => {
            debug(format!("Loading vk length: {:?}", l));
            l
        }
        Err(e) => {
            debug(format!("Loading vk error {:?}", e));
            return -1;
        }
    };
    let mut proof_buffer = [0u8; 8192];
    let proof_len = match load_witness(&mut proof_buffer, 0, 2, Source::Input) {
        Ok(l) => {
            debug(format!("Loading proof length: {:?}", l));
            l
        }
        Err(e) => {
            debug(format!("Loading proof error {:?}", e));
            return -1;
        }
    };

    let mut code_buffer = [0u8; 2048];
    let raw_code_len = match load_witness(&mut code_buffer, 0, 3, Source::Input) {
        Ok(l) => {
            debug(format!("Loading program length: {:?}", l));
            l
        }
        Err(e) => {
            debug(format!("Loading program error: {:?}", e));
            return -1;
        }
    };
    assert!(raw_code_len % 2 == 0); // san-check
    let code_len = raw_code_len / 2;
    let mut code = [Fr::zero(); 1024];
    code[0] = Fr::from(code_len as u64);
    (0..code_len).for_each(|idx| {
        code[idx + 1] =
            Fr::from(u16::from_le_bytes([code_buffer[idx * 2], code_buffer[idx * 2 + 1]]) as u64)
    });

    let mut input_buffer = [0u8; 1024];
    let input_len = match load_witness(&mut input_buffer, 0, 4, Source::Input) {
        Ok(l) => {
            debug(format!("Loading input length: {:?}", l));
            l
        }
        Err(e) => {
            debug(format!("Loading input error: {:?}", e));
            return -1;
        }
    };
    let mut input = [Fr::zero(); 1024];
    input[0] = Fr::from(input_len as u64);
    (0..input_len).for_each(|idx| {
        input[idx + 1] = Fr::from(input_buffer[idx] as u64);
    });

    let verifier_params = {
        let r: io::Result<ParamsVerifierKZG<Bn256>> =
            read_verifier_params(&mut &params_buffer[..params_len]);
        if r.is_err() {
            debug(format!(
                "Error on ParamsVerifierKZG::<Bn256>::read: {:?}",
                r.err()
            ));
            return -1;
        }
        r.unwrap()
    };

    let vk = {
        let r = VerifyingKey::<G1Affine>::read::<&[u8], MainCircuit<Fr>>(
            &mut &vk_buffer[..vk_len],
            halo2_proofs::SerdeFormat::RawBytes,
        );
        if r.is_err() {
            debug(format!("Error on VerifyingKey::read: {:?}", r.err()));
            return -1;
        };
        r.unwrap()
    };

    // Prepare instances
    let instances = [&code[0..(code_len + 1)], &input[0..(input_len + 1)]];

    let mut verifier_transcript =
        Blake2bRead::<_, G1Affine, Challenge255<_>>::init(&proof_buffer[..proof_len]);
    let strategy = SingleStrategy::new(&verifier_params);
    let res = verify_proof::<
        KZGCommitmentScheme<Bn256>,
        VerifierSHPLONK<'_, Bn256>,
        Challenge255<G1Affine>,
        Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
        SingleStrategy<'_, Bn256>,
    >(
        &verifier_params,
        &vk,
        strategy,
        &[&instances],
        &mut verifier_transcript,
    );
    if res.is_err() {
        debug(format!("Error on verify_proof: {:?}", res.err()));
        return -2;
    };
    debug(format!("Verifying successfully"));
    0
}
