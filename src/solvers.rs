use crate::hamiltonian::TwoLocalHamiltonian;
use crate::types::{
    CompactState, ComparableEnergy, Energy, ExternalMagneticField, Interactions, SpinIndex, State,
    Temperature,
};
use indexmap::IndexSet;
use ordered_float::{Float, OrderedFloat};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn gray_code(n: SpinIndex) -> SpinIndex {
    n ^ (n >> 1)
}

fn bit_position_changed(a: SpinIndex, b: SpinIndex) -> Option<u32> {
    let diff = a ^ b;
    if diff == 0 {
        None
    } else {
        Some(diff.trailing_zeros())
    }
}

fn from_compact_state_to_state(compact_state: CompactState) -> State {
    let mut state = vec![false; compact_state.len()];
    for index in compact_state.into_ones() {
        state[index] = true;
    }

    return state;
}

/// Finds all ground states of the spin glass whose interaction terms and external magnetic field
/// are given as the `interactions` and `external_magnetic_field` arguments.
///
/// ### Example
///
/// ```
/// use ernst::solvers::find_all_ground_states;
///
/// let s0 = 0;
/// let z = 1;
///
/// let copy_gate_interactions = vec![(s0, z, 1.0)];
/// let copy_gate_external_magnetic_field = vec![0.0, 0.0];
///
/// let actual_states = find_all_ground_states(&copy_gate_interactions, &copy_gate_external_magnetic_field);
/// let expected_states = vec![(-1.0, vec![false, false]), (-1.0, vec![true, true])];
///
/// assert_eq!(expected_states, actual_states)
/// ```
pub fn find_all_ground_states(
    interactions: &Interactions,
    external_magnetic_field: &ExternalMagneticField,
) -> Vec<(Energy, State)> {
    let n = external_magnetic_field.len();
    let initial_state = CompactState::with_capacity(n);
    let mut two_local_hamiltonian = TwoLocalHamiltonian::new(
        interactions.clone(),
        external_magnetic_field.clone(),
        Some(vec![false; n]),
    );

    let initial_energy = two_local_hamiltonian.current_energy();
    let mut lowest_energy = initial_energy;
    let mut ground_states: Vec<(Energy, CompactState)> = vec![(initial_energy, initial_state)];

    for i in 1..(1 << n) {
        let prev_gray = gray_code(i - 1);
        let curr_gray = gray_code(i);
        if let Some(bit_pos) = bit_position_changed(prev_gray, curr_gray) {
            two_local_hamiltonian.flip_spin(bit_pos as usize);
        }
        let current_energy = two_local_hamiltonian.current_energy();
        if (current_energy - lowest_energy).abs() < f32::EPSILON {
            ground_states.push((current_energy, two_local_hamiltonian.spins.clone()));
        } else if current_energy < lowest_energy {
            lowest_energy = current_energy;
            ground_states.clear();
            ground_states.push((current_energy, two_local_hamiltonian.spins.clone()));
        }
    }

    ground_states
        .into_iter()
        .map(|(energy, ground_state)| (energy, from_compact_state_to_state(ground_state)))
        .collect()
}

/// Parameters for simulated annealing.
/// - `initial_temperature`: temperature at the zeroth step
/// - `final_temperature`: temperature at the <sweeps> step
/// - `sweeps`: number of sampling steps
/// - `seed`: rng seed that ensures the whole process to be repeatable
/// - `trace`: if true, then it will keep track of all states found on the way to the ground state
pub struct SimulatedAnnealingConfiguration {
    pub initial_temperature: f32,
    pub final_temperature: f32,
    pub sweeps: usize,
    pub seed: u64,
    pub trace: bool,
}

impl Default for SimulatedAnnealingConfiguration {
    fn default() -> Self {
        SimulatedAnnealingConfiguration {
            initial_temperature: 273.15,
            final_temperature: 0.015,
            sweeps: 1000,
            seed: 42,
            trace: false,
        }
    }
}

pub type Epoch = usize;

/// Explores the energy landscape of the spin glass whose interaction terms and external magnetic field
/// are given as the `interactions` and `external_magnetic_field` arguments.
///
/// It will return the encountered states of lowest energy. See [SimulatedAnnealingConfiguration] for
/// information on how to make it so that it will return every single lowest energy state found.
///
/// ### Example
///
/// ```
/// use indexmap::map::VacantEntry;
/// use ernst::solvers::simulated_annealing;
///
/// let s0 = 0;
/// let z = 1;
///
/// let copy_gate_interactions = vec![(s0, z, 1.0)];
/// let copy_gate_external_magnetic_field = vec![0.0, 0.0];
///
/// let actual_states: Vec<_> = simulated_annealing(&copy_gate_interactions, &copy_gate_external_magnetic_field, None)
///   .into_iter()
///   .map(|(energy, state, _epoch)| (energy, state))
///   .collect();
/// let expected_states = vec![(-1.0, vec![false, false]), (-1.0, vec![true, true])];
///
/// assert_eq!(expected_states, actual_states)
/// ```
pub fn simulated_annealing(
    interactions: &Interactions,
    external_magnetic_field: &ExternalMagneticField,
    configuration_override: Option<&SimulatedAnnealingConfiguration>,
) -> Vec<(Energy, State, Epoch)> {
    let mut config = SimulatedAnnealingConfiguration::default();
    if let Some(configuration_override) = configuration_override {
        config.initial_temperature = configuration_override.initial_temperature;
        config.final_temperature = configuration_override.final_temperature;
        config.sweeps = configuration_override.sweeps;
        config.seed = configuration_override.seed;
    }
    let mut rng = StdRng::seed_from_u64(config.seed);
    let initial_temperature: Temperature = OrderedFloat::from(config.initial_temperature);
    let final_temperature: Temperature = OrderedFloat::from(config.final_temperature);
    let one = OrderedFloat::from(1.0);
    let cooling_rate = (final_temperature / initial_temperature).powf(one / config.sweeps as Energy);
    let mut temperature: Temperature = initial_temperature;
    let k = one.clone();

    let n = external_magnetic_field.len();
    let mut two_local_hamiltonian = TwoLocalHamiltonian::new(
        interactions.clone(),
        external_magnetic_field.clone(),
        Some(vec![false; n]),
    );

    let initial_energy: ComparableEnergy =
        OrderedFloat::from(two_local_hamiltonian.current_energy());
    let mut lowest_energy: ComparableEnergy = initial_energy;
    let mut ground_states: IndexSet<(ComparableEnergy, CompactState), ahash::RandomState> =
        vec![(initial_energy, two_local_hamiltonian.spins.clone())]
            .into_iter()
            .collect();
    let mut ground_state_update_time = vec![0];

    let zero = OrderedFloat::epsilon();
    for sweep in 1..config.sweeps {
        let spin_to_flip = rng.gen_range(0..two_local_hamiltonian.spins.len());
        let current_energy: ComparableEnergy = two_local_hamiltonian.current_energy().into();

        two_local_hamiltonian.flip_spin(spin_to_flip);
        let new_energy: ComparableEnergy = two_local_hamiltonian.current_energy().into();
        let delta_energy: ComparableEnergy = new_energy - current_energy;

        let not_acceptance_probability = OrderedFloat::from(rng.gen::<Energy>());
        let acceptance_probability = (-delta_energy / (k * temperature)).exp();
        if delta_energy <= zero || acceptance_probability > not_acceptance_probability
        {
            let new_ground_state = (new_energy, two_local_hamiltonian.spins.clone());
            if new_energy < lowest_energy {
                lowest_energy = new_energy;
                if !config.trace {
                    ground_states.clear();
                    ground_states.insert(new_ground_state);
                } else {
                    ground_states.insert(new_ground_state);
                }
                ground_state_update_time.push(sweep);
            } else if (new_energy - lowest_energy).abs() <= zero {
                if !ground_states.contains(&new_ground_state) {
                    ground_states.insert((new_energy, two_local_hamiltonian.spins.clone()));
                    ground_state_update_time.push(sweep);
                }
            }
        } else {
            two_local_hamiltonian.flip_spin(spin_to_flip);
        }

        temperature *= cooling_rate;
    }

    if !config.trace {
        ground_state_update_time = ground_state_update_time
            .drain(
                (ground_state_update_time.len() - ground_states.len())
                    ..ground_state_update_time.len(),
            )
            .collect()
    }

    ground_states
        .into_iter()
        .enumerate()
        .map(|(index, (energy, ground_state))| {
            (
                energy.into_inner(),
                from_compact_state_to_state(ground_state),
                ground_state_update_time[index],
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::solvers::{
        find_all_ground_states, simulated_annealing, SimulatedAnnealingConfiguration,
    };
    use crate::types::{ExternalMagneticField, Interactions};
    use ahash::HashSet;

    #[test]
    fn test_compute_all_states_and() {
        let s1 = 0;
        let s2 = 1;
        let s3 = 2;
        let s1_s3 = (s1, s3, 1.0);
        let s2_s3 = (s2, s3, 1.0);
        let s1_s2 = (s1, s2, -0.5);
        let interactions: Interactions = vec![s1_s2, s1_s3, s2_s3];
        let external_magnetic_field: ExternalMagneticField = vec![0.5, 0.5, -1.0];
        let actual_states = find_all_ground_states(&interactions, &external_magnetic_field);
        let expected_states = vec![
            (-1.5, vec![false, false, false]),
            (-1.5, vec![true, false, false]),
            (-1.5, vec![false, true, false]),
            (-1.5, vec![true, true, true]),
        ];

        assert_eq!(expected_states, actual_states)
    }

    #[test]
    fn test_compute_all_states_copy() {
        let s1 = 0;
        let s2 = 1;
        let interactions: Interactions = vec![(s1, s2, 1.0)];
        let external_magnetic_field: ExternalMagneticField = vec![0.0, 0.0];
        let actual_states = find_all_ground_states(&interactions, &external_magnetic_field);
        let expected_states = vec![(-1.0, vec![false, false]), (-1.0, vec![true, true])];

        assert_eq!(expected_states, actual_states)
    }

    #[test]
    fn test_compute_all_states_copy_copy_copy() {
        let interactions: Interactions = vec![(0, 1, 1.0), (1, 2, 1.0), (2, 3, 1.0)];
        let external_magnetic_field: ExternalMagneticField = vec![0.0, 0.0, 0.0, 0.0];
        let actual_states = find_all_ground_states(&interactions, &external_magnetic_field);
        let expected_states = vec![
            (-3.0, vec![false, false, false, false]),
            (-3.0, vec![true, true, true, true]),
        ];

        assert_eq!(expected_states, actual_states)
    }

    #[test]
    fn test_compute_all_states_copy_and_then_and() {
        // s1 -> s1' -> |
        //        |     s3 -> s3_prime
        // s2 -> s2' -> |
        let s1 = 0;
        let s1_prime = 1;
        let s2 = 2;
        let s2_prime = 3;
        let s3 = 4;
        let s3_prime = 5;
        let mut interactions: Interactions = vec![(s1, s1_prime, 1.0), (s2, s2_prime, 1.0)];
        interactions.push((s1_prime, s2_prime, -0.5));
        interactions.push((s1_prime, s3, 1.0));
        interactions.push((s2_prime, s3, 1.0));
        interactions.push((s3, s3_prime, 1.0));
        let external_magnetic_field: ExternalMagneticField = vec![0.0, 0.5, 0.0, 0.5, -1.0, 0.0];
        let actual_states = find_all_ground_states(&interactions, &external_magnetic_field);
        let expected_states = vec![
            (-4.5, vec![false, false, false, false, false, false]),
            (-4.5, vec![true, true, false, false, false, false]),
            (-4.5, vec![false, false, true, true, false, false]),
            (-4.5, vec![true, true, true, true, true, true]),
        ];

        assert_eq!(expected_states, actual_states)
    }

    #[test]
    fn test_compute_all_states_or() {
        let s1 = 0;
        let s2 = 1;
        let s3 = 2;
        let s1_s3 = (s1, s3, 1.0);
        let s2_s3 = (s2, s3, 1.0);
        let s1_s2 = (s1, s2, -0.5);
        let interactions: Interactions = vec![s1_s3, s2_s3, s1_s2];
        let external_magnetic_field: ExternalMagneticField = vec![-0.5, -0.5, 1.0];
        let actual_states = find_all_ground_states(&interactions, &external_magnetic_field);
        let expected_states = vec![
            (-1.5, vec![false, false, false]),
            (-1.5, vec![false, true, true]),
            (-1.5, vec![true, true, true]),
            (-1.5, vec![true, false, true]),
        ];

        assert_eq!(expected_states, actual_states)
    }

    #[test]
    fn test_compute_all_states_ternary_or_chain() {
        // s1 -> |
        // |     s3 -> s3' |
        // s2 -> |     |   |
        //             |   s5
        //             |   |
        // s4 -----------> |
        let s1 = 0;
        let s2 = 1;
        let s3 = 2;
        let s3_prime = 3;
        let s4 = 4;
        let s5 = 5;
        let interactions: Interactions = vec![
            (s1, s2, -0.5),
            (s1, s3, 1.0),
            (s2, s3, 1.0),
            (s3, s3_prime, 1.0),
            (s3_prime, s4, -0.5),
            (s4, s5, 1.0),
            (s3_prime, s5, 1.0),
        ];
        let external_magnetic_field: ExternalMagneticField = vec![-0.5, -0.5, 1.0, -0.5, -0.5, 1.0];
        let actual_states = find_all_ground_states(&interactions, &external_magnetic_field);
        let expected_states = vec![
            // 0, 0, 0, 0
            (-4.0, vec![false, false, false, false, false, false]),
            // 0, 0, 1, 1
            (-4.0, vec![false, false, false, false, true, true]),
            // 1, 0, 1, 1
            (-4.0, vec![true, false, true, true, true, true]),
            // 1, 1, 1, 1
            (-4.0, vec![true, true, true, true, true, true]),
            // 0, 1, 1, 1
            (-4.0, vec![false, true, true, true, true, true]),
            // 0, 1, 0, 1
            (-4.0, vec![false, true, true, true, false, true]),
            // 1, 1, 0, 1
            (-4.0, vec![true, true, true, true, false, true]),
            // 1, 0, 0, 1
            (-4.0, vec![true, false, true, true, false, true]),
        ];

        assert_eq!(expected_states, actual_states)
    }

    #[test]
    fn test_simulated_annealing_chained_or() {
        let s1 = 0;
        let s2 = 1;
        let s3 = 2;
        let s3_prime = 3;
        let s4 = 4;
        let s5 = 5;
        let interactions: Interactions = vec![
            (s1, s2, -0.5),
            (s1, s3, 1.0),
            (s2, s3, 1.0),
            (s3, s3_prime, 1.0),
            (s3_prime, s4, -0.5),
            (s4, s5, 1.0),
            (s3_prime, s5, 1.0),
        ];
        let external_magnetic_field: ExternalMagneticField = vec![-0.5, -0.5, 1.0, -0.5, -0.5, 1.0];
        let simulated_annealing_configuration = &SimulatedAnnealingConfiguration {
            initial_temperature: 1.0,
            final_temperature: 0.001,
            sweeps: 10000,
            seed: 42,
            trace: false,
        };
        let actual_states: HashSet<_> = simulated_annealing(
            &interactions,
            &external_magnetic_field,
            Some(&simulated_annealing_configuration),
        )
        .into_iter()
        .map(|(_energy, state, _epoch)| state)
        .collect();

        let expected_states: HashSet<_> = vec![
            vec![false, false, false, false, false, false],
            vec![false, false, false, false, true, true],
            vec![true, false, true, true, true, true],
            vec![true, true, true, true, true, true],
            vec![false, true, true, true, true, true],
            vec![false, true, true, true, false, true],
            vec![true, true, true, true, false, true],
            vec![true, false, true, true, false, true],
        ]
        .into_iter()
        .collect();

        let actual_diff = expected_states
            .difference(&actual_states)
            .into_iter()
            .collect::<Vec<_>>();
        let expected_diff: Vec<&Vec<bool>> = vec![];
        assert_eq!(actual_diff, expected_diff);
    }
}
