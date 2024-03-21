use std::time::Instant;
use ernst::spin_network::{OR, SpinNetwork};
use ernst::types::{ExternalMagneticField, Interactions};

fn main() {
    let some_h: ExternalMagneticField = vec![
        5.0, 5.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, -1.0, 0.5, 0.5, -1.0, 0.5, 0.5, -1.0, 0.5, 0.5, -1.0,
    ];
    let some_J: Interactions = vec![
        (0, 7, 1.0),
        (1, 8, 1.0),
        (7, 8, -0.5),
        (7, 9, 1.0),
        (8, 9, 1.0),
        (1, 10, 1.0),
        (2, 11, 1.0),
        (10, 11, -0.5),
        (10, 12, 1.0),
        (11, 12, 1.0),
        (0, 13, 1.0),
        (4, 14, 1.0),
        (13, 14, -0.5),
        (13, 15, 1.0),
        (14, 15, 1.0),
        (3, 16, 1.0),
        (2, 17, 1.0),
        (16, 17, -0.5),
        (16, 18, 1.0),
        (17, 18, 1.0),
        (3, 9, 1.0),
        (4, 12, 1.0),
        (5, 15, 1.0),
        (6, 18, 1.0),
    ];

    let now = Instant::now();
    let exact_ground_states = ernst::solvers::find_all_ground_states(&some_J, &some_h);
    let time_to_compute = now.elapsed().as_millis();
    for ground_state in exact_ground_states {
        println!("Energy: {} - State: {:?} - Took: {} ms", ground_state.0, ground_state.1, time_to_compute);
    }

    let now = Instant::now();
    let approximate_ground_states = ernst::solvers::simulated_annealing(&some_J, &some_h, None);
    let time_to_compute = now.elapsed().as_millis();
    for ground_state in approximate_ground_states {
        println!("Energy: {} - State: {:?} - Found in sweep number: {} - Took: {} ms", ground_state.0, ground_state.1, ground_state.2, time_to_compute);
    }

    let mut spin_network = SpinNetwork::new();
    let s0 = spin_network.add_input_node(0.0);
    let s1 = spin_network.add_input_node(0.0);
    let s2 = spin_network.add_input_node(0.0);
    let or_gate = OR::default();
    let z_aux = spin_network.add_binary_node(s0, s1, &or_gate);
    let z = spin_network.add_binary_node(z_aux, s2, &or_gate);

    let interesting_spins = vec![s0, s1, s2, z];
    let ternary_or_ground_states = spin_network.find_all_ground_states(Some(interesting_spins));
    println!("Ternary OR ground states:");
    for ground_state in ternary_or_ground_states {
        println!("Energy: {} - State: {:?}", ground_state.0, ground_state.1);
    }
}
