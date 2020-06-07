use std::path::Path;
use std::vec::Vec;
use image::{open, Rgba};

#[derive(Debug, Clone, Default)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub colours: Vec<u16>,
    pub data: Vec<u8>,
}

#[inline(always)]
const fn from_rgb(r: u16, g: u16, b: u16) -> u16 {
    (b & 0x1f) << 10 | (g & 0x1f) << 5 | (r & 0x1f)
}

fn convert_colour(colour: &Rgba<u8>) -> u16 {
    return from_rgb(
        (colour[0] >> 3) as u16,
        (colour[1] >> 2) as u16,
        (colour[2] >> 3) as u16
    );
}

pub fn read_image<P>(path: P) -> ImageInfo
where
    P: AsRef<Path>,
{
    let img = open(path).expect("Could not read file");
    let rgba_img = img.to_rgba();

    let mut colour_list: Vec<u16> = vec![0];
    let mut image_data: Vec<u8> = vec![];

    for pixel in rgba_img.pixels() {
        let converted = convert_colour(pixel);

        let index = colour_list
            .iter()
            .position(|&c| c == converted)
            .unwrap_or_else(|| {
                colour_list.push(converted);
                colour_list.len() - 1
            });

        image_data.push((index & 0x0f) as u8);
    }

    return ImageInfo{
        width: rgba_img.width(),
        height: rgba_img.height(),
        colours: colour_list,
        data: image_data,
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_read_image() {
        let img_info = read_image(Path::new("test_data/invader.png"));

        assert_eq!(img_info.width, 640);
        assert_eq!(img_info.height, 400);
        assert_eq!(img_info.colours.len(), 42);
        assert_eq!(img_info.data.len(), 640 * 400);
    }
}
