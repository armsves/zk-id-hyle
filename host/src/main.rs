// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use std::env;
use methods::{ HELLO_GUEST_ELF, HELLO_GUEST_ID };
use risc0_zkvm::{default_prover, ExecutorEnv, sha::Digestible};
use serde_json;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let args: Vec<String> = env::args().collect();
    let input: u32 = args[1].parse().expect("Please provide a valid integer as input");

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();
    let prover = default_prover();

    let receipt = prover
        .prove(env, HELLO_GUEST_ELF)
        .unwrap();

    let receipt_json = serde_json::to_string(&receipt).unwrap();
    std::fs::write("proof.json", receipt_json).unwrap();

    let output: (String, u32) = receipt.journal.decode().unwrap();

    println!("Group: {}", output.0);
    println!("Method ID: {:?} (hex)", receipt.inner.get_claim().unwrap().pre.digest());

    receipt
        .verify(HELLO_GUEST_ID)
        .unwrap();
}
