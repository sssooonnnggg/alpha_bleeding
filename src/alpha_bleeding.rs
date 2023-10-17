use image::{ImageResult, Rgba, RgbaImage};

struct Position {
    x: u32,
    y: u32,
}

struct AlphaBleeder {
    width: u32,
    height: u32,
    processed_pixels_map: Vec<bool>,
    pending_clear_pixels: Vec<Position>,
}

trait RgbaAccessor {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
    fn a(&self) -> u8;
}

impl RgbaAccessor for Rgba<u8> {
    fn r(&self) -> u8 {
        self[0]
    }
    fn g(&self) -> u8 {
        self[1]
    }
    fn b(&self) -> u8 {
        self[2]
    }
    fn a(&self) -> u8 {
        self[3]
    }
}

impl AlphaBleeder {
    fn run(&mut self, target: &mut RgbaImage) {
        let mut next_pass_pixels = self.setup(target);
        while !next_pass_pixels.is_empty() {
            next_pass_pixels = self.run_single_pass(next_pass_pixels, target)
        }
    }
    fn setup(&mut self, target: &mut RgbaImage) -> Vec<Position> {
        (self.width, self.height) = (target.width(), target.height());
        self.reset_processed_pixels_map();

        let mut next_pass_pixels = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if target.get_pixel(x, y).a() == 0 {
                    if AlphaBleeder::has_opaque_pixel_around(target, x, y) {
                        next_pass_pixels.push(Position { x, y });
                        self.mark_pixel_processed(x, y);
                    }
                } else {
                    self.mark_pixel_processed(x, y);
                }
            }
        }
        next_pass_pixels
    }

    fn reset_processed_pixels_map(&mut self) {
        self.processed_pixels_map = vec![false; (self.width * self.height) as usize];
    }

    fn mark_pixel_processed(&mut self, x: u32, y: u32) {
        let index = self.get_pixel_index(x, y);
        self.processed_pixels_map[index] = true;
    }

    fn is_pixel_processed(&self, x: u32, y: u32) -> bool {
        self.processed_pixels_map[self.get_pixel_index(x, y)]
    }

    fn get_pixel_index(&self, x: u32, y: u32) -> usize {
        (self.width * y + x) as usize
    }

    fn run_single_pass(&mut self, pixels: Vec<Position>, target: &mut RgbaImage) -> Vec<Position> {}

    fn get_pixels_around(image: &RgbaImage, x: u32, y: u32) -> Vec<(Rgba<u8>, u32, u32)> {
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
                let pixel = image.get_pixel(x, y);
                pixels.push((*pixel, x, y));
            }
        }
        pixels
    }

    fn has_opaque_pixel_around(image: &RgbaImage, x: u32, y: u32) -> bool {
        AlphaBleeder::get_pixels_around(image, x, y)
            .iter()
            .any(|p| p.0.a() > 0)
    }
}

fn perform_alpha_bleeding(target: &mut RgbaImage) {
    let mut processed_pixels = vec![false; target.width() as usize * target.height() as usize];
    let mut pending_process_pixels = Vec::new();
    let mut pending_clear_pixels = Vec::new();

    for y in 0..target.height() {
        for x in 0..target.width() {
            let pixel = target.get_pixel(x, y);
            if pixel[3] == 0 {
                if get_pixels_around(target, x, y).iter().any(|p| p.0[3] > 0) {
                    pending_process_pixels.push((x, y));
                    processed_pixels[(y * target.width() + x) as usize] = true;
                }
            } else {
                processed_pixels[(target.width() * y + x) as usize] = true;
            }
        }
    }

    let mut pending_process_next_pass = Vec::new();
    while !pending_process_pixels.is_empty() {
        let mut pending_modify_pixels = Vec::new();
        for (x, y) in pending_process_pixels.iter() {
            let pixels_around = get_pixels_around(target, *x, *y);
            let (mut r, mut g, mut b) = (0, 0, 0);
            let mut count = 0;
            for (pixel, x, y) in pixels_around.iter() {
                let index = (y * target.width() + x) as usize;
                if pixel[3] > 0 {
                    r += pixel[0] as u32;
                    g += pixel[1] as u32;
                    b += pixel[2] as u32;
                    count += 1;
                } else {
                    if !processed_pixels[index] {
                        processed_pixels[index] = true;
                        pending_process_next_pass.push((*x, *y));
                    }
                }
            }
            let (r, g, b) = ((r / count) as u8, (g / count) as u8, (b / count) as u8);
            pending_modify_pixels.push((*x, *y, Rgba([r, g, b, 255])));
            pending_clear_pixels.push((*x, *y));
        }
        for (x, y, pixel) in pending_modify_pixels.iter() {
            target.put_pixel(*x, *y, *pixel);
        }
        std::mem::swap(&mut pending_process_pixels, &mut pending_process_next_pass);
        pending_process_next_pass.clear();
    }

    for (x, y) in pending_clear_pixels {
        target.get_pixel_mut(x, y)[3] = 0;
    }
}

pub fn perform_alpha_bleeding_aux(from: &str, to: &str) -> ImageResult<()> {
    let source = image::open(from)?.into_rgba8();
    let mut target = source.clone();
    perform_alpha_bleeding(&mut target);
    target.save(to)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_alpha_bleeding() {
        let original = "tests/original.png";
        let output = "tests/alpha_bleeding_using_rs.png";
        let compared = "tests/alpha_bleeding_using_d.png";
        let result = perform_alpha_bleeding_aux(original, output);
        assert!(matches!(result, Ok(_)));
        let output = image::open(output).unwrap().into_rgba8();
        let expected = image::open(compared).unwrap().into_rgba8();
        assert_eq!(output, expected);
    }
}
