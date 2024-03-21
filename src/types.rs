use fixedbitset::FixedBitSet;
use ordered_float::OrderedFloat;

pub type Energy = f32;
pub type Temperature = OrderedFloat<f32>;
pub type ComparableEnergy = OrderedFloat<Energy>;
pub type SpinIndex = usize;
pub type InteractionStrength = f32;
pub type Interactions = Vec<(SpinIndex, SpinIndex, InteractionStrength)>;
pub type LinearizedUpperTriangularMatrix = Vec<Energy>;
pub type MagneticFieldStrength = f32;
pub type ExternalMagneticField = Vec<MagneticFieldStrength>;
pub type State = Vec<bool>;
pub type CompactState = FixedBitSet;
