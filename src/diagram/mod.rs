mod animate;
mod complex;
#[allow(clippy::module_inception)]
mod diagram;
mod generic;
mod simple;

pub use animate::AnimateDiagram;
pub use complex::ComplexDiagram;
pub use diagram::Diagram;
pub use generic::GenericDiagram;
pub use simple::SimpleDiagram;

#[cfg(feature = "serde")]
pub use generic::{Matrix, ToMatrix};

type Result<T = ()> = std::result::Result<T, crate::Error>;
