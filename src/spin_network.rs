use crate::solvers::{
    find_all_ground_states, simulated_annealing, Epoch, SimulatedAnnealingConfiguration,
};
use crate::types::{
    BinaryNode, TernaryNode, Energy, ExternalMagneticField, Interactions, MagneticFieldStrength, NAryNode, SpinIndex, State, UnaryNode
};

/// A SpinNetwork is meant to represent a 2D Spin Glass.
/// It provides methods to add any number of nodes with one, two, or n inputs, and one output.
#[derive(Default)]
pub struct SpinNetwork {
    input_nodes: Vec<SpinIndex>,
    auxiliary_nodes: Vec<SpinIndex>,
    output_nodes: Vec<SpinIndex>,
    pub interactions: Interactions,
    pub external_magnetic_field: ExternalMagneticField,
}

impl SpinNetwork {
    /// Creates a new SpinNetwork with no nodes, interactions or external magnetic field
    pub fn new() -> Self {
        return Default::default();
    }
    fn add_free_node(&mut self) -> usize {
        self.external_magnetic_field.push(0.0);

        self.external_magnetic_field.len() - 1
    }
    /// Adds an output node. You will most likely only use this method if you want to implement your own [Node].
    pub fn add_output_node(&mut self, magnetic_field_strength: MagneticFieldStrength) -> usize {
        let node_index = self.add_free_node();
        self.output_nodes.push(node_index);
        *self.external_magnetic_field.get_mut(node_index).unwrap() = magnetic_field_strength;

        node_index
    }
    /// Adds an auxiliary node. You will most likely only use this method if you want to implement your own [Node].
    pub fn add_auxiliary_node(&mut self, magnetic_field_strength: MagneticFieldStrength) -> usize {
        let node_index = self.add_free_node();
        self.auxiliary_nodes.push(node_index);
        *self.external_magnetic_field.get_mut(node_index).unwrap() = magnetic_field_strength;

        node_index
    }
    /// Adds an input node. If you add a positive magnetic field to the input node, it will act like a classical circuit
    /// with an input set to 1. You need input nodelib.
    ///
    /// ### Example
    ///
    /// ```
    /// use ernst::spin_network::SpinNetwork;
    ///
    /// let mut spin_network = SpinNetwork::new();
    /// let s0 = spin_network.add_input_node(2.0);
    ///
    /// assert_eq!(spin_network.external_magnetic_field[0], 2.0)
    /// ```
    pub fn add_input_node(&mut self, magnetic_field_strength: MagneticFieldStrength) -> usize {
        let node_index = self.add_free_node();
        self.input_nodes.push(node_index);
        *self.external_magnetic_field.get_mut(node_index).unwrap() = magnetic_field_strength;

        node_index
    }
    /// Adds a Node with a single input and output. It returns the index of the output node.
    ///
    /// ### Example
    ///
    /// ```
    /// use ernst::spin_network::SpinNetwork;
    /// use ernst::nodelib::logic_gates::COPY;
    ///
    /// let mut spin_network = SpinNetwork::new();
    /// let s0 = spin_network.add_input_node(0.0);
    /// let copy_gate = COPY::default();
    ///
    /// spin_network.add_unary_node(s0, &copy_gate);
    /// ```
    pub fn add_unary_node(&mut self, input: usize, unary_node: &impl UnaryNode) -> usize {
        return UnaryNode::connect_to_one(unary_node, self, input);
    }
    /// Adds a Node with two inputs and one output. It returns the index of the output node.
    ///
    /// ### Example
    ///
    /// ```
    /// use ernst::spin_network::SpinNetwork;
    /// use ernst::nodelib::logic_gates::AND;
    ///
    /// let mut spin_network = SpinNetwork::new();
    /// let s0 = spin_network.add_input_node(0.0);
    /// let s1 = spin_network.add_input_node(0.0);
    /// let and_gate = AND::default();
    ///
    /// spin_network.add_binary_node(s0, s1, &and_gate);
    /// ```
    pub fn add_binary_node(
        &mut self,
        left_input: usize,
        right_input: usize,
        binary_node: &impl BinaryNode,
    ) -> usize {
        return BinaryNode::connect_to_two(binary_node, self, left_input, right_input);
    }
    pub fn add_ternary_node(
        &mut self,
        first_input: usize,
        second_input: usize,
        third_input: usize,
        ternary_node: &impl TernaryNode,
    ) -> usize {
        return TernaryNode::connect_to_three(ternary_node, self, first_input, second_input, third_input);
    }
    // problem here: how to make it accept variable n number of inputs in rust?
     pub fn add_NAry_node(
        &mut self,
        inputs: &Vec<usize>,
        nary_node: &impl NAryNode,
    ) -> usize {
        NAryNode::connect_to_n(nary_node, self, inputs)
    }

    /// Finds all ground states of the spin glass represented by the SpinNetwork. The argument `spin_ordering`, when
    /// given, will ensure that the `State`s will be projected according to it.
    ///
    /// ### Example
    ///
    /// ```
    /// use ernst::spin_network::SpinNetwork;
    /// use ernst::nodelib::logic_gates::OR;
    ///
    /// let mut spin_network = SpinNetwork::new();
    /// let s0 = spin_network.add_input_node(0.0);
    /// let s1 = spin_network.add_input_node(0.0);
    /// let s2 = spin_network.add_input_node(0.0);
    ///
    /// let or_gate = OR::default();
    /// let z_aux = spin_network.add_binary_node(s0, s1, &or_gate);
    /// let z = spin_network.add_binary_node(z_aux, s2, &or_gate);
    ///
    /// // Note how we only ask for ground states to be ordered according to the "interesting" spins i.e
    /// // the ones that are able to
    /// let actual_ground_states = spin_network.find_all_ground_states(Some(vec![s0, s1, s2, z]));
    /// let expected_ground_states = vec![
    ///    (-7.0, vec![false, false, false, false]),
    ///    (-7.0, vec![true, false, false, true]),
    ///    (-7.0, vec![true, true, false, true]),
    ///    (-7.0, vec![false, true, false, true]),
    ///    (-7.0, vec![false, true, true, true]),
    ///    (-7.0, vec![true, true, true, true]),
    ///    (-7.0, vec![true, false, true, true]),
    ///    (-7.0, vec![false, false, true, true]),
    /// ];
    ///
    /// assert_eq!(expected_ground_states, actual_ground_states)
    /// ```
    pub fn find_all_ground_states(
        &self,
        spin_ordering: Option<Vec<SpinIndex>>,
    ) -> Vec<(Energy, State)> {
        return find_all_ground_states(&self.interactions, &self.external_magnetic_field)
            .into_iter()
            .map(|(energy, state)| {
                if let Some(spin_ordering) = &spin_ordering {
                    return (
                        energy,
                        spin_ordering
                            .iter()
                            .map(|spin_index| state[*spin_index])
                            .collect(),
                    );
                }

                return (energy, state);
            })
            .collect();
    }
    /// Explores the energy landscape of the spin glass represented by the SpinNetwork. The argument `spin_ordering`, when
    /// given, will ensure that the `State`s will be projected according
    /// to it.
    ///
    /// ### Example
    ///
    /// ```
    /// use ernst::spin_network::SpinNetwork;
    /// use ernst::nodelib::logic_gates::OR;
    ///
    /// let mut spin_network = SpinNetwork::new();
    /// let s0 = spin_network.add_input_node(0.0);
    /// let s1 = spin_network.add_input_node(0.0);
    /// let s2 = spin_network.add_input_node(0.0);
    ///
    /// let or_gate = OR::default();
    /// let z_aux = spin_network.add_binary_node(s0, s1, &or_gate);
    /// let z = spin_network.add_binary_node(z_aux, s2, &or_gate);
    ///
    /// // Note how we only ask for ground states to be ordered according to the "interesting" spins i.e
    /// // the ones that are able to
    /// let actual_ground_states = spin_network.run_simulated_annealing(None, Some(vec![s0, s1, s2, z]));
    /// let expected_ground_states = vec![
    ///    (-7.0, vec![false, false, false, false]),
    ///    (-7.0, vec![true, false, false, true]),
    ///    (-7.0, vec![true, true, false, true]),
    ///    (-7.0, vec![false, true, false, true]),
    ///    (-7.0, vec![false, true, true, true]),
    ///    (-7.0, vec![true, true, true, true]),
    ///    (-7.0, vec![true, false, true, true]),
    ///    (-7.0, vec![false, false, true, true]),
    /// ];
    ///
    /// assert_eq!(expected_ground_states, actual_ground_states)
    /// ```
    pub fn run_simulated_annealing(
        &self,
        configuration_override: Option<&SimulatedAnnealingConfiguration>,
        spin_ordering: Option<Vec<SpinIndex>>,
    ) -> Vec<(Energy, State, Epoch)> {
        return simulated_annealing(
            &self.interactions,
            &self.external_magnetic_field,
            configuration_override,
        )
        .into_iter()
        .map(|(energy, state, epoch)| {
            if let Some(spin_ordering) = &spin_ordering {
                return (
                    energy,
                    spin_ordering
                        .iter()
                        .map(|spin_index| state[*spin_index])
                        .collect(),
                    epoch,
                );
            }

            (energy, state, epoch)
        })
        .collect();
    }
    /// Returns the external magnetic field with flipped signs. The output of this function alongside `inverted_interactions`
    /// should be all that you need to find the ground state of this Spin Glass on a real quantum annealer.
    pub fn inverted_external_magnetic_field(&self) -> ExternalMagneticField {
        return self.external_magnetic_field.iter().map(|&energy| -energy).collect();
    }
    /// Returns the interaction terms flipped signs. The output of this function alongside `inverted_external_magnetic_field`
    /// should be all that you need to find the ground state of this Spin Glass on a real quantum annealer.
    pub fn inverted_interactions(&self) -> Interactions {
        return self.interactions.iter().map(|(left_spin_index, right_spin_index, energy)| (*left_spin_index, *right_spin_index, -(*energy))).collect();
    }
}
