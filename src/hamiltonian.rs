use crate::types::{CompactState, Energy, ExternalMagneticField, Interactions, LinearizedUpperTriangularMatrix, SpinIndex, State};
use ftree::FenwickTree;

pub(crate) struct TwoLocalHamiltonian {
    pub(crate) spins: CompactState,
    linearized_interactions: LinearizedUpperTriangularMatrix,
    external_magnetic_field: ExternalMagneticField,
    interaction_energy: FenwickTree<Energy>,
    magnetic_field_energy: FenwickTree<Energy>,
}

impl TwoLocalHamiltonian {
    fn map_interaction_to_index(i: SpinIndex, j: SpinIndex, n: usize) -> usize {
        (i * (2 * n - i - 1) / 2) + (j - i - 1)
    }

    pub fn new(
        interactions: Interactions,
        external_magnetic_field: ExternalMagneticField,
        initial_state: Option<State>,
    ) -> Self {
        let n = interactions
            .iter()
            .flat_map(|(i, j, _)| std::iter::once(i).chain(std::iter::once(j)))
            .max()
            .unwrap()
            + 1;
        assert_eq!(external_magnetic_field.len(), n);

        let mut spins = CompactState::with_capacity(n);
        if let Some(initial_spins) = initial_state {
            assert_eq!(
                initial_spins.len(),
                n,
                "The initial state has a different number of spins than what was found in the interaction vector"
            );
            for (index, spin) in initial_spins.iter().enumerate() {
                if *spin {
                    spins.toggle(index);
                }
            }
        };

        let magnetic_field_strength_values: Vec<Energy> = (0..n)
            .map(|i| {
                if spins.contains(i) {
                    external_magnetic_field[i].into()
                } else {
                    (-external_magnetic_field[i]).into()
                }
            })
            .collect();
        let magnetic_field_energy = FenwickTree::from_iter(magnetic_field_strength_values);

        let total_interactions = n * (n - 1) / 2;
        let mut linearized_interactions = vec![0.0; total_interactions];

        for (i, j, interaction_strength) in interactions.iter() {
            let smaller = std::cmp::min(i, j);
            let greater = std::cmp::max(i, j);
            let index = TwoLocalHamiltonian::map_interaction_to_index(*smaller, *greater, n);
            let i_spin_value = if spins.contains(*i) { 1.0 } else { -1.0 };
            let j_spin_value = if spins.contains(*j) { 1.0 } else { -1.0 };
            linearized_interactions[index] = interaction_strength * i_spin_value * j_spin_value;
        }
        let interaction_energy = FenwickTree::from_iter(linearized_interactions.clone());

        TwoLocalHamiltonian {
            spins,
            linearized_interactions,
            external_magnetic_field,
            interaction_energy,
            magnetic_field_energy,
        }
    }

    pub fn flip_spin(&mut self, spin: SpinIndex) {
        self.spins.toggle(spin);

        let sign_change = if self.spins.contains(spin) { 2.0 } else { -2.0 };

        let n = self.spins.len();

        for j in 0..n {
            if j != spin {
                let index = TwoLocalHamiltonian::map_interaction_to_index(
                    std::cmp::min(spin, j),
                    std::cmp::max(spin, j),
                    n,
                );
                if let Some(interaction_strength) = self.linearized_interactions.get(index) {
                    let other_spin_sign = if self.spins.contains(j) { 1.0 } else { -1.0 };
                    self.interaction_energy
                        .add_at(index, interaction_strength * sign_change * other_spin_sign);
                }
            }
        }

        if let Some(magnetic_field_strength) = self.external_magnetic_field.get(spin) {
            self.magnetic_field_energy
                .add_at(spin, sign_change * magnetic_field_strength);
        }
    }

    pub fn current_energy(&self) -> Energy {
        let interaction_energy = self
            .interaction_energy
            .prefix_sum(self.interaction_energy.len(), 0.0);
        let external_magnetic_field_energy = self
            .magnetic_field_energy
            .prefix_sum(self.magnetic_field_energy.len(), 0.0);

        return -interaction_energy + -external_magnetic_field_energy;
    }
}

#[cfg(test)]
mod tests {
    use crate::hamiltonian::TwoLocalHamiltonian;
    use crate::types::{ExternalMagneticField, Interactions};

    #[test]
    fn test_total_energy() {
        let interactions: Interactions = vec![(0, 1, -1.0), (1, 2, 2.0), (0, 2, 2.0)];
        let external_magnetic_field: ExternalMagneticField = vec![-1.0, -1.0, -3.0];

        let mut hamiltonian = TwoLocalHamiltonian::new(interactions, external_magnetic_field, None);

        assert_eq!(-8.0, hamiltonian.current_energy());

        hamiltonian.flip_spin(0);
        assert_eq!(-4.0, hamiltonian.current_energy());

        hamiltonian.flip_spin(1);
        assert_eq!(4.0, hamiltonian.current_energy());

        hamiltonian.flip_spin(2);
        assert_eq!(2.0, hamiltonian.current_energy());
    }
}
