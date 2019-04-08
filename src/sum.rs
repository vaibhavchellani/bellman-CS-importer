// just create one constraint
#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman;
extern crate pairing;
extern crate rand;

// For randomness (during paramgen and proof generation)
use self::rand::{thread_rng, Rng};

// Bring in some tools for using pairing-friendly curves
use self::pairing::{Engine, Field, PrimeField};

// We're going to use the BLS12-381 pairing-friendly elliptic curve.
use self::pairing::bls12_381::{Bls12, Fr};

// We'll use these interfaces to construct our circuit.
use self::bellman::{Circuit, ConstraintSystem, SynthesisError};

// We're going to use the Groth16 proving system.
use self::bellman::groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
};
