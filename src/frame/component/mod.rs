
/// [UnderlyingPixel] holds all supported inner pixel format that we support
pub mod rgb8;
pub mod ycbcr8;

use rgb8::Rgb8;
use ycbcr8::YCbCr8;

#[derive(Debug, Clone, Copy)]
pub enum UnderlyingComponent {
    /// Gray Scale 8 bit format
    Gray8(u8),
    /// [Rgb8] representation of a pixel
    Rgb8(Rgb8),
    /// [YCbCr8] representation of a pixel
    YCbCr8(YCbCr8),
}

impl Default for UnderlyingComponent {
    fn default() -> Self {
        Self::Rgb8(Default::default())
    }
}

impl UnderlyingComponent {

    /// Builds a new [Rgb8] [UnderlyingComponent]
    pub fn rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb8(Rgb8 { r, g, b})
    }

    /// Builds a new [UnderlyingComponent::Gray8]
    pub fn gray8(gray: u8) -> Self {
        Self::Gray8(gray)
    }

    /// Converts [UnderlyingComponent] to Gray8 (whatever the input format)
    pub(crate) fn to_gray8(&self) -> u8 {
        match self {
            Self::Gray8(gray8) => *gray8,
            Self::Rgb8(rgb8) => rgb8.to_gray8(),
            Self::YCbCr8(ycbcr8) => ycbcr8.to_rgb8().to_gray8(),
        }
    }
}
