use image::{Rgba, RgbaImage, ImageResult};

fn get_pixels_around(image: &RgbaImage, x: u32, y: u32) -> Vec<Rgba<u8>> {
    let mut pixels = Vec::new();
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            let x = x as i32 + i;
            let y = y as i32 + j;
            if x < 0 || y < 0 {
                continue;
            }
            let x = x as u32;
            let y = y as u32;
            if x >= image.width() || y >= image.height() {
                continue;
            }
            pixels.push(*image.get_pixel(x, y));
        }
    }
    pixels
}

fn is_edge_pixel(image: &RgbaImage, pixel: &Rgba<u8>, x: u32, y: u32) -> bool {
    pixel[3] == 0 && get_pixels_around(image, x, y).iter().any(|p| p[3] > 0)
}

fn perform_alpha_bleeding(source: &RgbaImage, target: &mut RgbaImage) {
    for x in 0..source.width() {
        for y in 0..source.height() {
            let pixel = source.get_pixel(x, y);
            if is_edge_pixel(source, pixel, x, y) {
                let pixels_around = get_pixels_around(source, x, y);
                let (mut r, mut g, mut b) = (0, 0, 0);
                let count = pixels_around
                    .iter()
                    .map(|pixel| {
                        r += pixel[0] as u32;
                        g += pixel[1] as u32;
                        b += pixel[2] as u32;
                    })
                    .count() as u32;
                let (r, g, b) = ((r / count) as u8, (g / count) as u8, (b / count) as u8);
                target.put_pixel(x, y, Rgba([r, g, b, 0]));
            }
        }
    }
}

pub fn perform_alpha_bleeding_aux(from: &str, to: &str) -> ImageResult<()> {
    let source = image::open(from)?.into_rgba8();
    let mut target = source.clone();
    perform_alpha_bleeding(&source, &mut target);
    target.save(to)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_alpha_bleeding() {
        let output = "tests/alpha_bleeding_using_rs.png";
        let result = perform_alpha_bleeding_aux("tests/original.png", output);
        assert!(matches!(result, Ok(_)));
        // let original = image::open("tests/original.png").unwrap().into_rgba8();
        // let output = image::open(output).unwrap().into_rgba8();
        // let expected = image::open("tests/alpha_bleeding_using_d.png").unwrap().into_rgba8();
        // for x in 0..output.width() {
        //     for y in 0..output.height() {
        //         let pixel1 = output.get_pixel(x, y);
        //         let pixel2 = expected.get_pixel(x, y);
        //         let original_pixel = original.get_pixel(x, y);
        //         if pixel1 != pixel2 {
        //             println!("position: {}, {}, original: {:?}, output: {:?}, expected: {:?}", x, y, original_pixel, pixel1, pixel2);
        //         }
        //     }
        // }
    }
}