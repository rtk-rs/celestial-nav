use crate::frame::component::Rgb8;

#[derive(Debug, Clone, Copy)]
/// [YCbCr8] representation of a pixel
pub struct YCbCr8 {
    /// Y luminance component
    pub y: u8,
    /// Cb: Blue chrominance component
    pub cb: u8,
    /// Cr: Red chrominance component
    pub cr: u8,
}

impl YCbCr8 {


    // pub(crate) fn max(&self) -> YCbCr8 {
    //     YCbCr8 {
    //         y: u8::MAX,
    //         cb: u8::MAX,
    //         cr: u8::MAX,
    //     }
    // }

    /// Converts [YCbCr8] to [Rgb8]
    pub(crate) fn to_rgb8(&self) -> Rgb8 {

        let r = self.y as f32 + 1.402 * (self.cr - 128) as f32;
        
        let mut g = self.y as f32 - 0.344136 * (self.cb - 128) as f32;
        g -= 0.714136 * (self.cr - 128) as f32;

        let b = self.y as f32 + 1.772 * (self.cb - 128) as f32;

        Rgb8 { 
            r: r.round().clamp(0.0, 255.0) as u8,
            g: g.round().clamp(0.0, 255.0) as u8,
            b: b.round().clamp(0.0, 255.0) as u8,
        }
    }
}