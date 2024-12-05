use std::path::Path;

use celestial_nav::{
    frame::component::UnderlyingComponent,
    prelude::{BitMap, Frame},
};

use image::EncodableLayout;

use image::{open as image_open, DynamicImage};

/// Loads [BitMap] from a local gray scale image
pub fn load_gray_scale_bitmap<P: AsRef<Path>>(buf: &mut [UnderlyingComponent], path: P) {
    let img = image_open(path).unwrap();

    match img {
        DynamicImage::ImageLuma8(gray_img) => {
            let width = gray_img.width() as u16;
            let height = gray_img.height() as u16;
            println!("bitmap_from_gray_scale:width={},height={}", width, height);

            for (nth, byte) in gray_img.as_bytes().iter().enumerate() {
                buf[nth] = UnderlyingComponent::gray8(*byte);
            }
        }
        fmt => panic!("invalid img format: {:?}", fmt),
    }
}

#[test]
fn test_stars_finder() {
    const WIDTH: usize = 612;
    const HEIGHT: usize = 408;
    const XY: usize = WIDTH * HEIGHT;
    let mut map: [UnderlyingComponent; XY] = [UnderlyingComponent::Gray8(0); XY];
    load_gray_scale_bitmap(
        &mut map,
        &format!("{}/img/sky1.jpg", env!("CARGO_MANIFEST_DIR")),
    );

    let bitmap = BitMap::from_slice(WIDTH as u16, HEIGHT as u16,&map);
    let frame = Frame::new(WIDTH, HEIGHT, bitmap);

    let coords = frame.star_coordinates_finder::<8>();

    panic!("{:?}", coords);
    
}
