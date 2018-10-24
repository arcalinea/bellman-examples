extern crate bellman;
extern crate pairing;
extern crate rand;
use bellman::{Circuit, ConstraintSystem, SynthesisError};
use pairing::{Engine, Field, PrimeField};

mod cube; 
mod multiply;

fn main(){
    use pairing::bls12_381::{Bls12, Fr};
    use rand::thread_rng;
    use bellman::groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
    };

    println!("Hello world");
    
    let rng = &mut thread_rng();
    
    println!("Creating parameters...");
    
    // Create parameters for our circuit
    let params = {
        let c = multiply::MultiplyDemo::<Bls12> {
            a: None,
            // make option values as None for these variables, for paramgen
            // don't want to bake these nums into parameters
            b: None,
            c: None
        };

        generate_random_parameters(c, rng).unwrap()
    };
    
    // Prepare the verification key (for proof verification)
    let pvk = prepare_verifying_key(&params.vk);

    println!("Creating proofs...");
    
    let public_input = Fr::from_str("21");
    
    // Create an instance of circuit
    let c = multiply::MultiplyDemo::<Bls12> {
        a: Fr::from_str("7"),
        // when creating instance here, pass in Some of actual variables you're using
        b: Fr::from_str("3"),
        c: public_input
    };
    
    // Create a groth16 proof with our parameters.
    let proof = create_random_proof(c, &params, rng).unwrap();
    
    assert!(verify_proof(
        &pvk,
        &proof,
        &[public_input.unwrap()]
    ).unwrap());
}