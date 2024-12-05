/// [Rgb8] representation of a pixel
#[derive(Debug, Clone, Default, Copy)]
pub struct Rgb8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb8 {

    // pub(crate) fn max(&self) -> Rgb8 {
    //     Rgb8 {
    //         r: u8::MAX,
    //         g: u8::MAX,
    //         b: u8::MAX,
    //     }
    // }

    /// Converts [Rgb8] to Gray scale 8bit
    pub(crate) fn to_gray8(&self) -> u8 {
        let gray8 = 0.299 * self.r as f32 + 0.587 * self.g as f32 + 0.114 * self.b as f32;
        gray8.round() as u8
    }

}