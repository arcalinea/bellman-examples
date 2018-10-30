#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman;
extern crate pairing;
extern crate rand;

// For randomness (during paramgen and proof generation)
use self::rand::{thread_rng};

// Bring in some tools for using pairing-friendly curves
use self::pairing::{
    Engine,
    Field,
    PrimeField
};

// We're going to use the BLS12-381 pairing-friendly elliptic curve.
use self::pairing::bls12_381::{
    Bls12,
    Fr,
};

// We'll use these interfaces to construct our circuit.
use self::bellman::{
    Circuit,
    ConstraintSystem,
    SynthesisError
};

// We're going to use the Groth16 proving system.
use self::bellman::groth16::{
    Proof,
    generate_random_parameters,
    prepare_verifying_key,
    create_random_proof,
    verify_proof,
};

// demo circuit
// proving that I know a such that a * 3 = 21
pub struct MultiplyDemo<E: Engine> {
    pub a: Option<E::Fr>,
    pub b: Option<E::Fr>,
    pub c: Option<E::Fr>
}

// create a demo circuit by using the `Circuit` trait which
/// is used during paramgen and proving in order to
/// synthesize the constraint system.
impl <E: Engine> Circuit<E> for MultiplyDemo<E> {
    fn synthesize<CS: ConstraintSystem<E>>(
        self, 
        cs: &mut CS
    ) -> Result<(), SynthesisError>
    {
        
        // Allocate the first value (private)
        let a = cs.alloc(|| "a", || {
            self.a.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Allocate the second value (private)
        let b = cs.alloc(|| "b", || {
            self.b.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Allocate the third value (public)
        // allocating a public input uses alloc_input
        let c = cs.alloc_input(|| "c", || {
            self.c.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // a * b = c?
        cs.enforce(
            || "mult",
            |lc| lc + a,
            |lc| lc + b,
            |lc| lc + c
        );
        
        Ok(())
    }
}

#[test]
fn test_multiply(){
    // This may not be cryptographically safe, use
    // `OsRng` (for example) in production software.
    let rng = &mut thread_rng();
    
    println!("Creating parameters...");
    
    // Create parameters for our circuit
    let params = {
        let c = MultiplyDemo::<Bls12> {
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
    let c = MultiplyDemo::<Bls12> {
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




