use std::collections::{BTreeMap, BTreeSet};
use ernst::nodelib::logic_gates::XOR;
use ernst::solvers::SimulatedAnnealingConfiguration;
use ernst::spin_network::SpinNetwork;
use ernst::types::SpinIndex;

type BinaryVector = Vec<bool>;
type VectorReference = usize;
type HammingDistance = usize;

fn hamming_distance(a: &BinaryVector, b: &BinaryVector) -> HammingDistance {
    return a.iter().zip(b.iter()).filter(|(a_i, b_i)| a_i != b_i).count();
}

fn main() {
    let binary_vectors = vec![
        vec![false, false, true, true, false],
        vec![false, true, false, true, true],
        vec![false, false, false, false, false],
        vec![false, false, false, true, false],
        vec![true, false, false, true, true]
    ];
    // Classical kNN
    let mut classicalkNNresults: Vec<(VectorReference, BTreeSet<(VectorReference, HammingDistance)>)> = Default::default();
    binary_vectors
        .iter()
        .enumerate()
        .for_each(|(outer_vector_reference ,outer_vector)| {
            classicalkNNresults.push((outer_vector_reference, Default::default()));
            binary_vectors.iter()
                .enumerate()
                .for_each(|(inner_vector_reference, inner_vector)| {
                    let distance = hamming_distance(outer_vector, inner_vector);
                    classicalkNNresults[outer_vector_reference].1.insert((inner_vector_reference, distance));
                });
        });

    // Quantum Annealing Friendly kNN
    let mut knnSpinNetwork = SpinNetwork::new();
    // First we have to instantiate all binary vectors in the quantum annealer
    let binary_vectors_spin_indexes: Vec<_> = binary_vectors
        .iter()
        .map(|binary_vector| {
            let mut binary_vector_spin_indexes = vec![];
            // We do so by representing each bit with one qubit
            for spin in binary_vector {
                // If it is 1, we bias it to be up, else down
                let magnetic_field_strength = if *spin { 5.0 } else { -5.0 };
                let spin_index = knnSpinNetwork.add_input_node(magnetic_field_strength);
                binary_vector_spin_indexes.push(spin_index);
            }

            binary_vector_spin_indexes
        })
        .collect();

    let xor_gate = XOR::default();
    // We now have to create one XOR for each qubit interaction.
    // This is quite wasteful, but allows for maximum parallelism.
    // i.e if we have vectors
    // [1, 1, 0, 0]
    // [0, 1, 0, 0]
    // it will be laid out somewhere in the annealer as:
    // XOR 1 0
    // XOR 1 1
    // XOR 0 0
    // XOR 0 0
    // All of these are disconnected from each other.

    // We use this to keep track of the XOR of each bit pair
    let mut xor_output_spin_indexes: BTreeMap<SpinIndex, BTreeMap<SpinIndex, SpinIndex>> = Default::default();
    binary_vectors_spin_indexes
        .iter()
        .enumerate()
        .for_each(|(outer_vector_reference , outer_vector_spin_indexes)| {
            binary_vectors_spin_indexes
                .iter()
                .enumerate()
                // The output of kNN can be described by an upper triangular matrix, because distance is reflexive, symmetric, and transitive
                // so in order to save qubits, let's only calculate what is necessary
                .filter(|(inner_vector_reference, _)| *inner_vector_reference > outer_vector_reference)
                .for_each(|(_inner_vector_reference, inner_vector_spin_indexes)| {
                    for (outer_spin_index, inner_spin_index) in outer_vector_spin_indexes.iter().zip(inner_vector_spin_indexes) {
                        let xor_spin_index = knnSpinNetwork.add_binary_node(*outer_spin_index, *inner_spin_index, &xor_gate);
                        xor_output_spin_indexes
                            .entry(*outer_spin_index)
                            .or_default()
                            .insert(*inner_spin_index, xor_spin_index);

                        // Symmetry
                        xor_output_spin_indexes
                            .entry(*inner_spin_index)
                            .or_default()
                            .insert(*outer_spin_index, xor_spin_index);
                    }
                });
        });

    // The spins of interest are all XOR outputs
    let spins_of_interest: Vec<SpinIndex> = xor_output_spin_indexes
        .values()
        .flat_map(|value| value
            .values()
        )
        .copied()
        .collect();

    // By default we will run 10000 Annealing sweeps
    let annealing_configuration = SimulatedAnnealingConfiguration {
        initial_temperature: 273.15,
        final_temperature: 0.015,
        sweeps: 10000,
        seed: 42,
        trace: false,
    };
    let annealing_output = knnSpinNetwork.run_simulated_annealing(Some(&annealing_configuration), Some(spins_of_interest.clone()));
    // We should only have found ONE ground state. If this fails, then it will panic the program
    assert_eq!(annealing_output.len(), 1);
    println!("Energy: {}", annealing_output[0].0);
    println!("Number of spins: {}\n", knnSpinNetwork.external_magnetic_field.len());

    let final_state: BTreeMap<SpinIndex, bool> = spins_of_interest
        .into_iter()
        .enumerate()
        .map(|(position, xor_output_spin_index)| (xor_output_spin_index, annealing_output[0].1[position]))
        .collect();

    // Now we are ready to build the result
    let mut quantumkNNresults: Vec<(VectorReference, BTreeSet<(VectorReference, HammingDistance)>)> = Default::default();
    binary_vectors_spin_indexes
        .iter()
        .enumerate()
        .for_each(|(outer_vector_reference, outer_spin_indexes)| {
            quantumkNNresults.push((outer_vector_reference, Default::default()));

            binary_vectors_spin_indexes
                .iter()
                .enumerate()
                .for_each(|(inner_vector_reference, inner_spin_indexes)| {
                    let mut hamming_distance = 0;
                    if outer_vector_reference != inner_vector_reference {
                        for (outer_spin_index, inner_spin_index) in outer_spin_indexes.iter().zip(inner_spin_indexes) {
                            let xor_output_index = xor_output_spin_indexes
                                .get(outer_spin_index)
                                .unwrap()
                                .get(inner_spin_index)
                                .unwrap();

                            if *final_state.get(xor_output_index).unwrap() {
                                hamming_distance += 1;
                            }
                        };
                    }

                    quantumkNNresults[outer_vector_reference].1.insert((inner_vector_reference, hamming_distance));
                })
        });

    for (vector_reference, (classical_knn_result, quantum_knn_result)) in classicalkNNresults.iter().zip(&quantumkNNresults).enumerate() {
        println!("Distance from vector {} to all others (vector_index, distance)", vector_reference);
        println!("Classical knn: {:?}", classical_knn_result.1);
        println!("Quantum knn: {:?}\n", quantum_knn_result.1);
    }
}