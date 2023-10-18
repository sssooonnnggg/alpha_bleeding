use image::{ImageResult, Rgba, RgbaImage};

struct Position {
    x: u32,
    y: u32,
}

trait RgbaAccessor {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
    fn a(&self) -> u8;
    fn clear_alpha(&mut self);
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
    fn clear_alpha(&mut self) {
        self[3] = 0;
    }
}

#[derive(Default)]
struct AlphaBleeder {
    width: u32,
    height: u32,
    processed_pixels_map: Vec<bool>,
    pending_clear_pixels: Vec<Position>,
}

impl AlphaBleeder {
    fn run(&mut self, target: &mut RgbaImage) {
        let mut next_pass_pixels = self.setup(target);
        while !next_pass_pixels.is_empty() {
            next_pass_pixels = self.run_single_pass(next_pass_pixels, target)
        }
        self.process_pending_clear_pixels(target);
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

    fn run_single_pass(&mut self, pixels: Vec<Position>, target: &mut RgbaImage) -> Vec<Position> {
        let mut pending_bleeding_pixels = Vec::new();
        let mut next_pass_pixels = Vec::new();
        for Position { x, y } in pixels.into_iter() {
            let pixels_around = AlphaBleeder::get_pixels_around(target, x, y);
            let (mut r, mut g, mut b) = (0, 0, 0);
            let mut count = 0;
            for (pixel, x, y) in pixels_around.into_iter() {
                if pixel.a() > 0 {
                    r += pixel.r() as u32;
                    g += pixel.g() as u32;
                    b += pixel.b() as u32;
                    count += 1;
                } else if !self.is_pixel_processed(x, y) {
                    self.mark_pixel_processed(x, y);
                    next_pass_pixels.push(Position { x, y });
                }
            }
            let (r, g, b) = ((r / count) as u8, (g / count) as u8, (b / count) as u8);
            pending_bleeding_pixels.push((x, y, Rgba([r, g, b, 255])));
            self.pending_clear_pixels.push(Position { x, y });
        }
        for (x, y, pixel) in pending_bleeding_pixels.iter() {
            target.put_pixel(*x, *y, *pixel);
        }
        next_pass_pixels
    }

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

    fn process_pending_clear_pixels(&mut self, target: &mut RgbaImage) {
        for &Position { x, y } in self.pending_clear_pixels.iter() {
            target.get_pixel_mut(x, y).clear_alpha()
        }
    }
}

pub fn perform_alpha_bleeding_aux(from: &str, to: &str) -> ImageResult<()> {
    let source = image::open(from)?.into_rgba8();
    let mut target = source.clone();
    let mut bleeder = AlphaBleeder::default();
    bleeder.run(&mut target);
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
        assert!(result.is_ok());
        let output = image::open(output).unwrap().into_rgba8();
        let expected = image::open(compared).unwrap().into_rgba8();
        assert_eq!(output, expected);
    }
}
