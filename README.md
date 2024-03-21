# Ernst: 2D Spin Glass Simulation

[![Crates.io](https://img.shields.io/crates/v/ernst)](https://crates.io/crates/ernst)
[![docs](https://img.shields.io/crates/v/ernst?color=yellow&label=docs)](https://docs.rs/ernst)

[Quantum Annealing](https://en.wikipedia.org/wiki/Adiabatic_quantum_computation) is an interesting way to leverage physical
quantum effects to solve NP-hard problems. It relies on encoding problems as a single 2-dimensional [Spin Glass](https://en.wikipedia.org/wiki/Spin_glass), such that
its ground states will map one-to-one to the solutions being sought.

As there is barely any software to help you simulate  

## Functionality

With `Ernst` you can:

1. Incrementally build a 2D spin glass, `SpinNetwork`, with a library of pre-built logic gates
2. Find its exact ground states with `find_all_ground_states` (only recommended if the number of spins is < 48)
3. Efficiently seek for ground states (with history) of potentially very large spin networks with `run_simulated_annealing` 
4. Get the `h` and `J` components of the `SpinNetwork` hamiltonian to send to `D-wave`

## Implementation

This library is optimized for incremental computation on a CPU. It makes no use of any linear algebra library. Its source of 
efficiency lies in cleverly doing incremental adjustments to the energy calculation with a [Fenwick Tree](https://en.wikipedia.org/wiki/Fenwick_tree).

## Why is it called Ernst?
The energy of a spin glass configuration is often computed according to the hamiltonian of a well-known statistical mechanics model, the [Ising Model](https://en.wikipedia.org/wiki/Ising_model), which is named after [Ernst Ising](https://en.wikipedia.org/wiki/Ernst_Ising).