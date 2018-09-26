# bellman tutorial 

Examples of circuits for [bellman](https://github.com/zkcrypto/bellman/), a Rust zk-SNARKs library.

Bellman provides a `Circuit` trait which you can use to synthesize the constraints in your program. 

To construct a circuit, first flatten your program into its constituent steps. 

Allocate the variables, then enforce the constraints. 

Enforcing the constraint takes the form of `A * B = C`. (is a linear combination, vectors of all your variables)

The `lc` in the `cs.enforce` function stands for "linear combination", and is an inner product of all the variables with some vector of coefficients.

### Generating Parameters 

Our examples use the function `generate_random_parameters` to generate a random set of parameters for testing. 

### Creating a proof

To create a proof, instantiate a version of the struct that is passed into the circuit, with the inputs to the circuit. 

We used the function `create_random_proof` to create a random groth16 proof. 

### Verifying a proof

To verify a proof, prepare the verifying key by passing in `params.vk` to `prepare_verifying_key`. This gives you the prepared viewing key, `pvk`.

The function `verify_proof` takes the prepared viewing key `pvk`, the `proof`, and the output as an array.

