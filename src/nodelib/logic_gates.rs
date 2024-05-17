use crate::spin_network::SpinNetwork;
use crate::types::{BinaryNode, TernaryNode, Energy, MagneticFieldStrength, Node, UnaryNode};

#[derive(Default)]
pub struct COPY {
    magnetic_field_strength: Energy,
}
impl COPY {
    fn new(magnetic_field_strength: MagneticFieldStrength) -> Self {
        return COPY {
            magnetic_field_strength,
        };
    }
}
impl Node for COPY {
    fn connect(&self, spin_network: &mut SpinNetwork) -> usize {
        spin_network.add_output_node(self.magnetic_field_strength)
    }
}
impl UnaryNode for COPY {
    fn connect_to_one(&self, spin_network: &mut SpinNetwork, input: usize) -> usize {
        let output_node_index = self.connect(spin_network);
        spin_network
            .interactions
            .push((input, output_node_index, 1.0));

        output_node_index
    }
}

#[derive(Default)]
pub struct NOT {}
impl Node for NOT {
    fn connect(&self, spin_network: &mut SpinNetwork) -> usize {
        spin_network.add_output_node(0.0)
    }
}
impl UnaryNode for NOT {
    fn connect_to_one(&self, spin_network: &mut SpinNetwork, input: usize) -> usize {
        let output_node_index = self.connect(spin_network);
        spin_network
            .interactions
            .push((input, output_node_index, -1.0));

        output_node_index
    }
}
#[derive(Default)]
pub struct AND {}
impl Node for AND {
    fn connect(&self, spin_network: &mut SpinNetwork) -> usize {
        spin_network.add_output_node(-1.0)
    }
}
impl BinaryNode for AND {
    fn connect_to_two(
        &self,
        spin_network: &mut SpinNetwork,
        left_input: usize,
        right_input: usize,
    ) -> usize {
        let output_node_index = self.connect(spin_network);
        let copy_with_half = COPY::new(0.5);
        let left_copy_output_index = spin_network.add_unary_node(left_input, &copy_with_half);
        let right_copy_output_index = spin_network.add_unary_node(right_input, &copy_with_half);

        let left_to_right = (left_copy_output_index, right_copy_output_index, -0.5);
        let left_to_output = (left_copy_output_index, output_node_index, 1.0);
        let right_to_output = (right_copy_output_index, output_node_index, 1.0);

        spin_network.interactions.push(left_to_right);
        spin_network.interactions.push(left_to_output);
        spin_network.interactions.push(right_to_output);

        output_node_index
    }
}
#[derive(Default)]
pub struct OR {}
impl Node for OR {
    fn connect(&self, spin_network: &mut SpinNetwork) -> usize {
        spin_network.add_output_node(1.0)
    }
}
impl BinaryNode for OR {
    fn connect_to_two(
        &self,
        spin_network: &mut SpinNetwork,
        left_input: usize,
        right_input: usize,
    ) -> usize {
        let output_node_index = self.connect(spin_network);
        let copy_with_minus_half = COPY::new(-0.5);
        let left_copy_output_index = spin_network.add_unary_node(left_input, &copy_with_minus_half);
        let right_copy_output_index =
            spin_network.add_unary_node(right_input, &copy_with_minus_half);

        let left_to_right = (left_copy_output_index, right_copy_output_index, -0.5);
        let left_to_output = (left_copy_output_index, output_node_index, 1.0);
        let right_to_output = (right_copy_output_index, output_node_index, 1.0);

        spin_network.interactions.push(left_to_right);
        spin_network.interactions.push(left_to_output);
        spin_network.interactions.push(right_to_output);

        output_node_index
    }
}
impl TernaryNode for OR {
    fn connect_to_three(
        &self,
        spin_network: &mut SpinNetwork,
        first_input: usize,
        second_input: usize,
        third_input: usize,
    ) -> usize {
        let ouput_node_index: usize = self.connect(spin_network);
        let copy_with_minus_third = COPY::new(-1.0 / 3.0);
        let first_copy_output_index = spin_network.add_unary_node(first_input, &copy_with_minus_third);
        let second_copy_output_index =
            spin_network.add_unary_node(second_input, &copy_with_minus_third);
        let third_copy_output_index = spin_network.add_unary_node(third_input, &copy_with_minus_third);

        let first_to_second = (first_copy_output_index, second_copy_output_index, -1.0 / 3.0);
        let first_to_third = (first_copy_output_index, third_copy_output_index, -1.0 / 3.0);
        let second_to_third = (second_copy_output_index, third_copy_output_index, -1.0 / 3.0);
        let first_to_output = (first_copy_output_index, ouput_node_index, 1.0);
        let second_to_output = (second_copy_output_index, ouput_node_index, 1.0);
        let third_to_output = (third_copy_output_index, ouput_node_index, 1.0);

        spin_network.interactions.push(first_to_second);
        spin_network.interactions.push(first_to_third);
        spin_network.interactions.push(second_to_third);
        spin_network.interactions.push(first_to_output);
        spin_network.interactions.push(second_to_output);
        spin_network.interactions.push(third_to_output);

        ouput_node_index
    }

}//

//


#[derive(Default)]
pub struct NAND {}
impl Node for NAND {
    fn connect(&self, spin_network: &mut SpinNetwork) -> usize {
        spin_network.add_output_node(1.0)
    }
}
impl BinaryNode for NAND {
    fn connect_to_two(
        &self,
        spin_network: &mut SpinNetwork,
        left_input: usize,
        right_input: usize,
    ) -> usize {
        let output_node_index = self.connect(spin_network);
        let copy_with_minus_half = COPY::new(0.5);
        let left_copy_output_index = spin_network.add_unary_node(left_input, &copy_with_minus_half);
        let right_copy_output_index =
            spin_network.add_unary_node(right_input, &copy_with_minus_half);

        let left_to_right = (left_copy_output_index, right_copy_output_index, -0.5);
        let left_to_output = (left_copy_output_index, output_node_index, -1.0);
        let right_to_output = (right_copy_output_index, output_node_index, -1.0);

        spin_network.interactions.push(left_to_right);
        spin_network.interactions.push(left_to_output);
        spin_network.interactions.push(right_to_output);

        output_node_index
    }
}

#[derive(Default)]
pub struct NOR {}
impl Node for NOR {
    fn connect(&self, spin_network: &mut SpinNetwork) -> usize {
        spin_network.add_output_node(-1.0)
    }
}
impl BinaryNode for NOR {
    fn connect_to_two(
        &self,
        spin_network: &mut SpinNetwork,
        left_input: usize,
        right_input: usize,
    ) -> usize {
        let output_node_index = self.connect(spin_network);
        let copy_with_minus_half = COPY::new(-0.5);
        let left_copy_output_index = spin_network.add_unary_node(left_input, &copy_with_minus_half);
        let right_copy_output_index =
            spin_network.add_unary_node(right_input, &copy_with_minus_half);

        let left_to_right = (left_copy_output_index, right_copy_output_index, -0.5);
        let left_to_output = (left_copy_output_index, output_node_index, -1.0);
        let right_to_output = (right_copy_output_index, output_node_index, -1.0);

        spin_network.interactions.push(left_to_right);
        spin_network.interactions.push(left_to_output);
        spin_network.interactions.push(right_to_output);

        output_node_index
    }
}

#[derive(Default)]
pub struct XOR {}
impl Node for XOR {
    fn connect(&self, spin_network: &mut SpinNetwork) -> usize {
        spin_network.add_output_node(-0.5)
    }
}
impl BinaryNode for XOR {
    fn connect_to_two(
        &self,
        spin_network: &mut SpinNetwork,
        left_input: usize,
        right_input: usize,
    ) -> usize {
        let output_node_index = self.connect(spin_network);
        let aux_node_index = spin_network.add_auxiliary_node(-1.0);
        let copy_with_minus_half = COPY::new(-0.5);
        let left_copy_output_index = spin_network.add_unary_node(left_input, &copy_with_minus_half);
        let right_copy_output_index =
            spin_network.add_unary_node(right_input, &copy_with_minus_half);

        let left_to_right = (left_copy_output_index, right_copy_output_index, -0.5);

        let left_to_aux = (left_copy_output_index, aux_node_index, -1.0);
        let right_to_aux = (right_copy_output_index, aux_node_index, -1.0);

        let left_to_output = (left_copy_output_index, output_node_index, -0.5);
        let right_to_output = (right_copy_output_index, output_node_index, -0.5);
        let aux_to_output = (aux_node_index, output_node_index, -1.0);

        spin_network.interactions.push(left_to_right);
        spin_network.interactions.push(left_to_aux);
        spin_network.interactions.push(right_to_aux);
        spin_network.interactions.push(left_to_output);
        spin_network.interactions.push(right_to_output);
        spin_network.interactions.push(aux_to_output);

        output_node_index
    }
}

#[derive(Default)]
pub struct XNOR {}
impl Node for XNOR {
    fn connect(&self, spin_network: &mut SpinNetwork) -> usize {
        spin_network.add_output_node(0.5)
    }
}
impl BinaryNode for XNOR {
    fn connect_to_two(
        &self,
        spin_network: &mut SpinNetwork,
        left_input: usize,
        right_input: usize,
    ) -> usize {
        let output_node_index = self.connect(spin_network);
        let aux_node_index = spin_network.add_auxiliary_node(-1.0);
        let copy_with_minus_half = COPY::new(-0.5);
        let left_copy_output_index = spin_network.add_unary_node(left_input, &copy_with_minus_half);
        let right_copy_output_index =
            spin_network.add_unary_node(right_input, &copy_with_minus_half);

        let left_to_right = (left_copy_output_index, right_copy_output_index, -0.5);

        let left_to_aux = (left_copy_output_index, aux_node_index, -1.0);
        let right_to_aux = (right_copy_output_index, aux_node_index, -1.0);

        let left_to_output = (left_copy_output_index, output_node_index, 0.5);
        let right_to_output = (right_copy_output_index, output_node_index, 0.5);
        let aux_to_output = (aux_node_index, output_node_index, 1.0);

        spin_network.interactions.push(left_to_right);
        spin_network.interactions.push(left_to_aux);
        spin_network.interactions.push(right_to_aux);
        spin_network.interactions.push(left_to_output);
        spin_network.interactions.push(right_to_output);
        spin_network.interactions.push(aux_to_output);

        output_node_index
    }
}

#[cfg(test)]
mod tests {
    use crate::nodelib::logic_gates::{AND, COPY, NAND, NOR, NOT, OR, XNOR, XOR};
    use crate::spin_network::SpinNetwork;

    #[test]
    fn test_copy() {
        let mut spin_network = SpinNetwork::new();
        let s0 = spin_network.add_input_node(0.0);
        let copy_gate = COPY::new(0.0);
        let z = spin_network.add_unary_node(s0, &copy_gate);

        let actual_ground_states = spin_network.find_all_ground_states(Some(vec![s0, z]));
        let expected_ground_states = vec![(-1.0, vec![false, false]), (-1.0, vec![true, true])];

        assert_eq!(expected_ground_states, actual_ground_states)
    }

    #[test]
    fn test_not() {
        let mut spin_network = SpinNetwork::new();
        let s0 = spin_network.add_input_node(0.0);
        let not_gate = NOT::default();
        let z = spin_network.add_unary_node(s0, &not_gate);

        let actual_ground_states = spin_network.find_all_ground_states(Some(vec![s0, z]));
        let expected_ground_states = vec![(-1.0, vec![true, false]), (-1.0, vec![false, true])];

        assert_eq!(expected_ground_states, actual_ground_states)
    }

    #[test]
    fn test_and() {
        let mut spin_network = SpinNetwork::new();
        let s0 = spin_network.add_input_node(0.0);
        let s1 = spin_network.add_input_node(0.0);
        let and_gate = AND::default();
        let z = spin_network.add_binary_node(s0, s1, &and_gate);

        let actual_ground_states = spin_network.find_all_ground_states(Some(vec![s0, s1, z]));
        let expected_ground_states = vec![
            (-3.5, vec![false, false, false]),
            (-3.5, vec![true, false, false]),
            (-3.5, vec![true, true, true]),
            (-3.5, vec![false, true, false]),
        ];

        assert_eq!(expected_ground_states, actual_ground_states)
    }

    #[test]
    fn test_or() {
        let mut spin_network = SpinNetwork::new();
        let s0 = spin_network.add_input_node(0.0);
        let s1 = spin_network.add_input_node(0.0);
        let or_gate = OR::default();
        let z = spin_network.add_binary_node(s0, s1, &or_gate);

        let actual_ground_states = spin_network.find_all_ground_states(Some(vec![s0, s1, z]));
        let expected_ground_states = vec![
            (-3.5, vec![false, false, false]),
            (-3.5, vec![true, false, true]),
            (-3.5, vec![true, true, true]),
            (-3.5, vec![false, true, true]),
        ];

        assert_eq!(expected_ground_states, actual_ground_states)
    }

    #[test]
    fn test_nand() {
        let mut spin_network = SpinNetwork::new();
        let s0 = spin_network.add_input_node(0.0);
        let s1 = spin_network.add_input_node(0.0);
        let nand_gate = NAND::default();
        let z = spin_network.add_binary_node(s0, s1, &nand_gate);

        let actual_ground_states = spin_network.find_all_ground_states(Some(vec![s0, s1, z]));
        let expected_ground_states = vec![
            (-3.5, vec![false, false, true]),
            (-3.5, vec![true, false, true]),
            (-3.5, vec![true, true, false]),
            (-3.5, vec![false, true, true]),
        ];

        assert_eq!(expected_ground_states, actual_ground_states)
    }

    #[test]
    fn test_nor() {
        let mut spin_network = SpinNetwork::new();
        let s0 = spin_network.add_input_node(0.0);
        let s1 = spin_network.add_input_node(0.0);
        let nand_gate = NOR::default();
        let z = spin_network.add_binary_node(s0, s1, &nand_gate);

        let actual_ground_states = spin_network.find_all_ground_states(Some(vec![s0, s1, z]));
        let expected_ground_states = vec![
            (-3.5, vec![false, false, true]),
            (-3.5, vec![true, false, false]),
            (-3.5, vec![true, true, false]),
            (-3.5, vec![false, true, false]),
        ];

        assert_eq!(expected_ground_states, actual_ground_states)
    }

    #[test]
    fn test_xor() {
        let mut spin_network = SpinNetwork::new();
        let s0 = spin_network.add_input_node(0.0);
        let s1 = spin_network.add_input_node(0.0);
        let xor_gate = XOR::default();
        let z = spin_network.add_binary_node(s0, s1, &xor_gate);

        let actual_ground_states = spin_network.find_all_ground_states(Some(vec![s0, s1, z]));
        let expected_ground_states = vec![
            (-4.0, vec![false, false, false]),
            (-4.0, vec![true, false, true]),
            (-4.0, vec![true, true, false]),
            (-4.0, vec![false, true, true]),
        ];

        assert_eq!(expected_ground_states, actual_ground_states)
    }

    #[test]
    fn test_xnor() {
        let mut spin_network = SpinNetwork::new();
        let s0 = spin_network.add_input_node(0.0);
        let s1 = spin_network.add_input_node(0.0);
        let xnor_gate = XNOR::default();
        let z = spin_network.add_binary_node(s0, s1, &xnor_gate);

        let actual_ground_states = spin_network.find_all_ground_states(Some(vec![s0, s1, z]));
        let expected_ground_states = vec![
            (-4.0, vec![false, false, true]),
            (-4.0, vec![true, false, false]),
            (-4.0, vec![true, true, true]),
            (-4.0, vec![false, true, false]),
        ];

        assert_eq!(expected_ground_states, actual_ground_states)
    }

    #[test]
    fn test_ternary_or() {
        let mut spin_network = SpinNetwork::new();
        let s0 = spin_network.add_input_node(0.0);
        let s1 = spin_network.add_input_node(0.0);
        let s2 = spin_network.add_input_node(0.0);
        let or_gate = OR::default();
        let z_aux = spin_network.add_binary_node(s0, s1, &or_gate);
        let z = spin_network.add_binary_node(z_aux, s2, &or_gate);

        let actual_ground_states = spin_network.find_all_ground_states(Some(vec![s0, s1, s2, z]));
        let expected_ground_states = vec![
            (-7.0, vec![false, false, false, false]),
            (-7.0, vec![true, false, false, true]),
            (-7.0, vec![true, true, false, true]),
            (-7.0, vec![false, true, false, true]),
            (-7.0, vec![false, true, true, true]),
            (-7.0, vec![true, true, true, true]),
            (-7.0, vec![true, false, true, true]),
            (-7.0, vec![false, false, true, true]),
        ];

        assert_eq!(expected_ground_states, actual_ground_states)
    }
}
