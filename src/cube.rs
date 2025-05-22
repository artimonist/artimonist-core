/// Cube diagram
///   diagram implementation for multiple layers matrix
///
/// # Parameters
///   H: matrix height
///   W: matrix weight
///   L: matrix layers
///
pub trait CubeDiagram<const H: usize = 7, const W: usize = 7>: GenericDiagram<H, W> {}
