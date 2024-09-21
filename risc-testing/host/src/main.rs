use methods::{MULTIPLY_ELF, MULTIPLY_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
use serde::Serialize;
use risc0_ethereum_contracts::groth16;

#[derive(Serialize)]
struct Input {
    a: u32,
    b: u32,
    c: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let a: u32 = 37;
    let b: u32 = 67;
    let c: u32 = 11;

    let prover = default_prover();

    let input1 = Input{ a: a , b: b , c: c  };

    let env = ExecutorEnv::builder()
    .write(&input1).unwrap()
    .build()
    .unwrap();

    let receipt = prover.prove_with_ctx(env, &VerifierContext::default(), MULTIPLY_ELF, &ProverOpts::groth16())?.receipt;

    // Encode the seal with the selector.
    let seal = groth16::encode(receipt.inner.groth16()?.seal.clone())?;

    // Extract the journal from the receipt.
    let journal = receipt.journal.bytes.clone();

    // Save the seal and journal to a file.
    let _savedfile : () = match std::fs::write("./seal.json", seal) {
        Ok(_) => (),
        Err(e) => panic!("Failed to write file: {}", e),
    };

    let _savedfile2 : () = match std::fs::write("./journal.json", journal) {
        Ok(_) => (),
        Err(e) => panic!("Failed to write file: {}", e),
    };

    Ok(())

    // let prove_info = prover
    //     .prove(env, MULTIPLY_ELF)
    //     .unwrap();

    // extract the receipt.
    // let receipt = prove_info.receipt;

    // For example:
    // let _output: u32 = receipt.journal.decode().unwrap();

    // The receipt was verified at the end of proving, but the below code is an
    // example of how someone else could verify this receipt.
    // receipt
    //     .verify(MULTIPLY_ID)
    //     .unwrap();  

    // let serialized_proof = bincode::serialize(&receipt).unwrap();

    // let savedfile : () = match std::fs::write("./receipt.json", serialized_proof) {
    //     Ok(_) => (),
    //     Err(e) => panic!("Failed to write file: {}", e),
    // };
}