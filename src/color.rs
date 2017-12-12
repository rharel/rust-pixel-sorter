use palette::{Hsl, Hsv, FromColor, Rgb, RgbHue};
use std::str::FromStr;

/// Enumerates color spaces.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorSpace { HSL, HSV, RGB }

/// Enumerates components from all spaces.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorComponent
{
    Hue,
    Saturation,
    Lightness,
    Value,

    Red,
    Green,
    Blue
}
impl ColorComponent
{
    /// Gets the color space the component belongs to.
    pub fn color_space(&self) -> ColorSpace
    {
        match *self
        {
            ColorComponent::Hue        => ColorSpace::HSL,
            ColorComponent::Saturation => ColorSpace::HSL,
            ColorComponent::Lightness  => ColorSpace::HSL,
            ColorComponent::Value      => ColorSpace::HSV,

            ColorComponent::Red   => ColorSpace::RGB,
            ColorComponent::Green => ColorSpace::RGB,
            ColorComponent::Blue  => ColorSpace::RGB
        }
    }
    /// Gets the index of the component in a value triplet of the relevant color space.
    pub fn index(&self) -> usize
    {
        match *self
        {
            ColorComponent::Hue        => 0,
            ColorComponent::Saturation => 1,
            ColorComponent::Lightness  => 2,
            ColorComponent::Value      => 2,

            ColorComponent::Red   => 0,
            ColorComponent::Green => 1,
            ColorComponent::Blue  => 2
        }
    }
}
impl FromStr for ColorComponent
{
    type Err = ();

    fn from_str(source: &str) -> Result<ColorComponent, ()>
    {
        match source
        {
            "Hue"        => Ok(ColorComponent::Hue),
            "Saturation" => Ok(ColorComponent::Saturation),
            "Lightness"  => Ok(ColorComponent::Lightness),
            "Value"      => Ok(ColorComponent::Value),

            "Red"   => Ok(ColorComponent::Red),
            "Green" => Ok(ColorComponent::Green),
            "Blue"  => Ok(ColorComponent::Blue),

            _ => Err(())
        }
    }
}

/// Masks components of a color space.
#[derive(Clone, Debug)]
pub struct ColorSpaceMask
{
    pub space: ColorSpace,
    pub components: [bool; 3]
}
impl Default for ColorSpaceMask
{
    fn default() -> Self
    {
        ColorSpaceMask
        {
            space: ColorSpace::RGB,
            components: [false, false, false]
        }
    }
}
impl<'a> From<[&'a str; 3]> for ColorSpaceMask
{
    fn from(source: [&'a str; 3]) -> Self
    {
        let mut mask = ColorSpaceMask
        {
            space: ColorComponent::from_str(source[0])
                .unwrap_or(ColorComponent::Red)
                .color_space(),
            components: [false, false, false]
        };
        for i in 0..3
        {
            if let Ok(component) = ColorComponent::from_str(source[i])
            {
                if component.color_space() == mask.space
                {
                    mask.components[component.index()] = true;
                }
            }
        }
        mask
    }
}

/// A position in a generic color space.
#[derive(Clone, Debug)]
pub struct Color
{
    pub space: ColorSpace,
    pub components: [f32; 3]
}
impl Color
{
    /// Creates a new RGB float triplet from specified 8 bit components.
    pub fn rgb_from_u8(red: u8, green: u8, blue: u8) -> Self
    {
        Color
        {
            space: ColorSpace::RGB,
            components: [red   as f32 / 255.0,
                         green as f32 / 255.0,
                         blue  as f32 / 255.0]
        }
    }
    /// Converts to 8 bit components.
    pub fn to_u8(&self) -> [u8; 3]
    {
        [(self.components[0] * 255.0) as u8,
         (self.components[1] * 255.0) as u8,
         (self.components[2] * 255.0) as u8]
    }
    /// Converts to the specified space.
    pub fn to_space(&self, space: ColorSpace) -> Self
    {
        if space == self.space { return self.clone(); }
        Color
        {
            space,
            components: match self.space
            {
                ColorSpace::HSL => Color::hsl_to_space(&self.components, space),
                ColorSpace::HSV => Color::hsv_to_space(&self.components, space),
                ColorSpace::RGB => Color::rgb_to_space(&self.components, space)
            }
        }
    }
    fn hsl_to_space(source: &[f32;3], space: ColorSpace) -> [f32; 3]
    {
        let hsl = Hsl::new(
            RgbHue::from_radians(source[0]),
            source[1],
            source[2]
        );
        match space
        {
            ColorSpace::HSL => [hsl.hue.to_radians(), hsl.saturation, hsl.lightness],
            ColorSpace::HSV =>
            {
                let hsv = Hsv::from_hsl(hsl);
                [hsv.hue.to_radians(), hsv.saturation, hsv.value]
            },
            ColorSpace::RGB =>
            {
                let rgb = Rgb::from_hsl(hsl);
                [rgb.red, rgb.green, rgb.blue]
            }
        }
    }
    fn hsv_to_space(source: &[f32;3], space: ColorSpace) -> [f32; 3]
    {
        let hsv = Hsv::new(
            RgbHue::from_radians(source[0]),
            source[1],
            source[2]
        );
        match space
        {
            ColorSpace::HSL =>
            {
                let hsl = Hsl::from_hsv(hsv);
                [hsl.hue.to_radians(), hsl.saturation, hsl.lightness]
            },
            ColorSpace::HSV => [hsv.hue.to_radians(), hsv.saturation, hsv.value],
            ColorSpace::RGB =>
            {
                let rgb = Rgb::from_hsv(hsv);
                [rgb.red, rgb.green, rgb.blue]
            }
        }
    }
    fn rgb_to_space(source: &[f32;3], space: ColorSpace) -> [f32; 3]
    {
        let rgb = Rgb::new(
            source[0],
            source[1],
            source[2]
        );
        match space
        {
            ColorSpace::HSL =>
            {
                let hsl = Hsl::from_rgb(rgb);
                [hsl.hue.to_radians(), hsl.saturation, hsl.lightness]
            },
            ColorSpace::HSV =>
            {
                let hsv = Hsv::from_rgb(rgb);
                [hsv.hue.to_radians(), hsv.saturation, hsv.value]
            },
            ColorSpace::RGB => [rgb.red, rgb.green, rgb.blue]
        }
    }
}
