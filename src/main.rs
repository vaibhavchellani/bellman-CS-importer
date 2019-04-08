#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman;
extern crate pairing;
extern crate rand;
use bellman::{Circuit, ConstraintSystem, SynthesisError};
use pairing::{Engine, Field, PrimeField};
mod sum;

fn main() {
    use bellman::groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
    };
    use pairing::bls12_381::{Bls12, Fr};
    use rand::thread_rng;

    println!("Prove that I know x such that x+1.");

    let rng = &mut thread_rng();

    println!("Creating parameters...");

    // Create parameters for our circuit
    let params = {
        let c = sum::SumDemo::<Bls12> { x: None, out: None };

        generate_random_parameters(c, rng).unwrap()
    };

    // Prepare the verification key (for proof verification)
    let pvk = prepare_verifying_key(&params.vk);

    println!("Creating proofs...");

    // Create an instance of circuit
    let c = sum::SumDemo::<Bls12> {
        x: Fr::from_str("3"),
        out: Fr::from_str("4"),
    };

    // Create a groth16 proof with our parameters.
    let proof = create_random_proof(c, &params, rng).unwrap();

    assert!(verify_proof(&pvk, &proof, &[Fr::from_str("5").unwrap()]).unwrap());
}
