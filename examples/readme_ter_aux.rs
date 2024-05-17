use ernst::spin_network::SpinNetwork;
use ernst::types::{ExternalMagneticField, Interactions};
use std::time::Instant;
use ernst::nodelib::logic_gates::OR;

fn main() {
    let mut spin_network = SpinNetwork::new();
    let s0 = spin_network.add_input_node(0.0);
    let s1 = spin_network.add_input_node(0.0);
    let s2 = spin_network.add_input_node(0.0);
    let or_gate = OR::default();
    let z_aux = spin_network.add_binary_node(s0, s1, &or_gate);
    let z = spin_network.add_binary_node(z_aux, s2, &or_gate);

    let interesting_spins = vec![s0, s1, s2, z];
    let ternary_or_ground_states = spin_network.find_all_ground_states(Some(interesting_spins.clone()));
    println!("\nTernary OR spin glass ground states:");
    for ground_state in ternary_or_ground_states {
        println!("\tEnergy: {} - State: {:?}", ground_state.0, ground_state.1);
    }

    let copy_ground_states = spin_network.run_simulated_annealing(None, Some(interesting_spins));
    println!("\nTernary OR spin glass ground states with simulated annealing:");
    for ground_state in copy_ground_states {
        println!("\tEnergy: {} - State: {:?} - Found in sweep number: {}", ground_state.0, ground_state.1, ground_state.2);
    }
}
