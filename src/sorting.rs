use color::{Color, ColorComponent, ColorSpace, ColorSpaceMask};
use image::RgbImage;
use std::str::FromStr;

/// Enumerates possible dimensions to sort along..
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Dimension { Row, Column }
impl FromStr for Dimension
{
    type Err = ();

    fn from_str(source: &str) -> Result<Dimension, ()>
    {
        match source
        {
            "Row"    => Ok(Dimension::Row),
            "Column" => Ok(Dimension::Column),
            _ => Err(())
        }
    }
}

/// Groups options for a sorting operation.
#[derive(Clone, Debug)]
pub struct SortingOptions
{
    /// Component to sort by.
    pub comparison_component: ColorComponent,
    /// Dimension to sort along.
    pub dimension: Dimension,
    /// Components to affect.
    pub affected_components: ColorSpaceMask
}

/// Pixel-sorts the specified image in-place using the specified options.
pub fn sort_in_place(image: &mut RgbImage, options: &SortingOptions)
{
    let &SortingOptions
    {
        comparison_component,
        dimension,
        ref affected_components
    } = options;

    let (inner_size, outer_size) = get_size(image, dimension);

    let mut inner: Vec<Color> = Vec::with_capacity(inner_size as usize);
    inner.resize(inner_size as usize, Color::rgb_from_u8(0, 0, 0));

    for outer_index in 0..outer_size
    {
        // Convert pixels into comparison space
        for inner_index in 0..inner_size
        {
            let (x, y) = io_to_xy(inner_index, outer_index, dimension);
            let pixel = image.get_pixel(x, y).data;
            inner[inner_index as usize] =
                Color::rgb_from_u8(pixel[0], pixel[1], pixel[2])
                    .to_space(comparison_component.color_space());
        }

        // Sort by the specified component.
        inner.sort_unstable_by(|a, b|
        {
            use std::cmp::Ordering::Equal;

            a.components[comparison_component.index()]
                .partial_cmp(&b.components[comparison_component.index()])
                .unwrap_or(Equal)
        });

        // Affect pixels in affected space and convert back to u8 RGB.
        for inner_index in 0..inner_size
        {
            let (x, y) = io_to_xy(inner_index, outer_index, dimension);
            let pixel = image.get_pixel_mut(x, y);

            let source = inner[inner_index as usize].to_space(affected_components.space);
            let mut target = Color::rgb_from_u8(pixel.data[0], pixel.data[1], pixel.data[2])
                .to_space(affected_components.space);

            for i in 0..3
            {
                if affected_components.components[i]
                {
                    target.components[i] = source.components[i];
                }
            }

            let rgb = target.to_space(ColorSpace::RGB).to_u8();
            pixel.data[0] = rgb[0];
            pixel.data[1] = rgb[1];
            pixel.data[2] = rgb[2];
        }
    }
}
/// Returns the specified image's (width, height) when the specified dimension is rows, and
/// (height, width) otherwise.
fn get_size(image: &RgbImage, dimension: Dimension) -> (u32, u32)
{
    let inner_size;
    let outer_size;
    if let Dimension::Row = dimension
    {
        inner_size = image.width();
        outer_size = image.height();
    }
    else
    {
        inner_size = image.height();
        outer_size = image.width();
    }
    (inner_size, outer_size)
}
/// Converts dimension-relative inner/outer indices to x/y pixel coordinates.
fn io_to_xy(
    inner_index: u32,
    outer_index: u32,
    dimension: Dimension) -> (u32, u32)
{
    match dimension
    {
        Dimension::Row    => (inner_index, outer_index),
        Dimension::Column => (outer_index, inner_index)
    }
}
