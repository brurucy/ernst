# Ernst: 2D Spin Glass Simulation for quantum annealing

[![Crates.io](https://img.shields.io/crates/v/ernst)](https://crates.io/crates/ernst)
[![docs](https://img.shields.io/crates/v/ernst?color=yellow&label=docs)](https://docs.rs/ernst)

[Quantum Annealing](https://en.wikipedia.org/wiki/Adiabatic_quantum_computation) is an interesting way to leverage physical quantum effects to solve NP-hard problems. It 
relies on encoding problems as a single 2-dimensional [Spin Glass](https://en.wikipedia.org/wiki/Spin_glass), such that its degenerate ground states will 
map one-to-one to the solutions being sought.

The raison d'etre of this library is to make it easy for you to create spin glasses, simulate them, and then send them
over to a quantum annealer.

## Functionality

With `Ernst` you can:

1. Incrementally build a 2D spin glass with the extensible `SpinNetwork` struct, alongside a library of pre-built logic gates
2. Find its exact ground states with `find_all_ground_states` (only recommended if the number of spins is < 48)
3. Efficiently seek for ground states (with history) of potentially very large spin networks with `run_simulated_annealing` 
4. Get the `h` and `J` components of the `SpinNetwork` hamiltonian to send to `D-wave`

Here is an example:
```rust
use ernst::spin_network::SpinNetwork;
use ernst::types::{ExternalMagneticField, Interactions};
use std::time::Instant;
use ernst::nodelib::logic_gates::OR;

fn main() {
    let some_h: ExternalMagneticField = vec![
        5.0, 5.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, -1.0, 0.5, 0.5, -1.0, 0.5, 0.5, -1.0, 0.5,
        0.5, -1.0,
    ];
    let some_j: Interactions = vec![
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
    let exact_ground_states = ernst::solvers::find_all_ground_states(&some_j, &some_h);
    let time_to_compute = now.elapsed().as_millis();
    println!("Complex spin glass ground state - took: {} ms", time_to_compute);
    for ground_state in exact_ground_states {
        println!(
            "\tEnergy: {} - State: {:?}",
            ground_state.0, ground_state.1
        );
    }

    let now = Instant::now();
    let approximate_ground_states = ernst::solvers::simulated_annealing(&some_j, &some_h, None);
    let time_to_compute = now.elapsed().as_millis();
    println!("\nComplex spin glass ground state with simulated annealing - took: {} ms", time_to_compute);
    for ground_state in approximate_ground_states {
        println!(
            "\tEnergy: {} - State: {:?} - Found in sweep number: {}",
            ground_state.0, ground_state.1, ground_state.2
        );
    }

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
```

## Implementation

This library is optimized for incremental computation on a CPU. It does not make use of a linear algebra library. Its source of 
efficiency lies in cleverly doing incremental adjustments to the energy calculation with a [Fenwick Tree](https://en.wikipedia.org/wiki/Fenwick_tree).

## Why is it called Ernst?

The energy of a spin glass configuration is often computed according to the hamiltonian of a well-known statistical mechanics model, the [Ising Model](https://en.wikipedia.org/wiki/Ising_model), which is named after [Ernst Ising](https://en.wikipedia.org/wiki/Ernst_Ising).

## Roadmap

1. Parallel tempering
2. Bayesian Optimisation

## Ernst for commercial projects

Are you interested in using Ernst for a commercial quantum optimization project? Then hit me up at <github-username>@protonmail.ch

## Ernst for research

If you find this project useful for your research, you may cite it as follows:

```text
@misc{Rucy2024,
  author = {Rucy, B.C.D.L}
  title = {Ernst},
  year = {2024},
  publisher = {GitHub},
  journal = {GitHub repository},
  howpublished = {\url{https://github.com/brurucy/ernst}},
}
```

Additionally, you may consider adding a star to the repository. This positive feedback improves the visibility of the project.