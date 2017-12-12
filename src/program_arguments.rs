use color::{ColorComponent, ColorSpaceMask};
use sorting::Dimension;

/// Contains program options.
#[derive(Clone, Debug)]
pub struct ProgramArguments
{
    /// Path to the input image.
    pub input_file_path: String,
    /// Path to the output directory.
    pub output_path: String,
    /// Component to sort by.
    pub comparison_component: ColorComponent,
    /// Dimension to sort along.
    pub dimension: Dimension,
    /// Components to affect.
    pub affected_components: ColorSpaceMask,
    /// Indicates program verbosity level.
    pub is_verbose: bool
}
