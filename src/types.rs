use crate::spin_network::SpinNetwork;
use fixedbitset::FixedBitSet;
use ordered_float::OrderedFloat;

pub type Energy = f32;
pub type Temperature = OrderedFloat<Energy>;
pub type ComparableEnergy = OrderedFloat<Energy>;
pub type SpinIndex = usize;
pub type InteractionStrength = Energy;
pub type Interactions = Vec<(SpinIndex, SpinIndex, InteractionStrength)>;
pub type LinearizedUpperTriangularMatrix = Vec<Energy>;
pub type MagneticFieldStrength = Energy;
pub type ExternalMagneticField = Vec<MagneticFieldStrength>;
pub type State = Vec<bool>;
pub type CompactState = FixedBitSet;

/// A Node is anything that is able to connect itself to the spin network.
pub trait Node {
    fn connect(&self, spin_network: &mut SpinNetwork) -> SpinIndex;
}
pub trait UnaryNode: Node {
    fn connect_to_one(&self, spin_network: &mut SpinNetwork, input: SpinIndex) -> SpinIndex;
}
pub trait BinaryNode: Node {
    fn connect_to_two(
        &self,
        spin_network: &mut SpinNetwork,
        left_input: SpinIndex,
        right_input: SpinIndex,
    ) -> SpinIndex;
}
pub trait NAryNode: Node {
    fn connect_to_n(&self, spin_network: &mut SpinNetwork, inputs: &Vec<SpinIndex>) -> SpinIndex;
}
