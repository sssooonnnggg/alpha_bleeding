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
    fn is_transparent(&self) -> bool;
    fn set_transparent(&mut self);
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
    fn is_transparent(&self) -> bool {
        self[3] == 0
    }
    fn set_transparent(&mut self) {
        self[3] = 0;
    }
}

#[derive(Default)]
struct PixelMarks<T: Default + Copy> {
    width: u32,
    marks: Vec<T>,
}

impl<T> PixelMarks<T>
where
    T: Default + Copy,
{
    fn new(width: u32, height: u32) -> Self {
        PixelMarks {
            width,
            marks: vec![T::default(); (width * height) as usize],
        }
    }

    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn get(&self, x: u32, y: u32) -> T {
        let index = self.index(x, y);
        self.marks[index]
    }

    fn set(&mut self, x: u32, y: u32, mark: T) {
        let index = self.index(x, y);
        self.marks[index] = mark;
    }
}

#[derive(Default)]
struct AlphaBleeder {
    width: u32,
    height: u32,
    marks: PixelMarks<bool>,
    pending_clear: Vec<Position>,
}

impl AlphaBleeder {
    fn execute(&mut self, image: &mut RgbaImage) {
        let mut next = self.setup(image);
        while !next.is_empty() {
            next = self.single_pass(next, image)
        }
        self.finalize(image);
    }

    fn setup(&mut self, image: &mut RgbaImage) -> Vec<Position> {
        (self.width, self.height) = (image.width(), image.height());
        self.marks = PixelMarks::new(self.width, self.height);

        let mut next = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if image.get_pixel(x, y).is_transparent() {
                    if AlphaBleeder::has_opaque_neighbor(image, x, y) {
                        next.push(Position { x, y });
                        self.marks.set(x, y, true);
                    }
                } else {
                    self.marks.set(x, y, true);
                }
            }
        }
        next
    }

    fn single_pass(&mut self, pixels: Vec<Position>, image: &mut RgbaImage) -> Vec<Position> {
        let mut should_bleeding = Vec::new();
        let mut next = Vec::new();
        for Position { x, y } in pixels.into_iter() {
            let neighbors = AlphaBleeder::find_neighbors(image, x, y);
            let (mut r, mut g, mut b) = (0, 0, 0);
            let mut count = 0;
            for (pixel, x, y) in neighbors.into_iter() {
                if pixel.a() > 0 {
                    r += pixel.r() as u32;
                    g += pixel.g() as u32;
                    b += pixel.b() as u32;
                    count += 1;
                } else if !self.marks.get(x, y) {
                    self.marks.set(x, y, true);
                    next.push(Position { x, y });
                }
            }
            let (r, g, b) = ((r / count) as u8, (g / count) as u8, (b / count) as u8);
            should_bleeding.push((x, y, Rgba([r, g, b, 255])));
            self.pending_clear.push(Position { x, y });
        }
        for &(x, y, pixel) in should_bleeding.iter() {
            image.put_pixel(x, y, pixel);
        }
        next
    }

    fn find_neighbors(image: &RgbaImage, x: u32, y: u32) -> Vec<(Rgba<u8>, u32, u32)> {
        let mut neighbors = Vec::new();
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
                neighbors.push((*pixel, x, y));
            }
        }
        neighbors
    }

    fn has_opaque_neighbor(image: &RgbaImage, x: u32, y: u32) -> bool {
        AlphaBleeder::find_neighbors(image, x, y)
            .iter()
            .any(|p| p.0.a() > 0)
    }

    fn finalize(&mut self, image: &mut RgbaImage) {
        for &Position { x, y } in self.pending_clear.iter() {
            image.get_pixel_mut(x, y).set_transparent()
        }
    }
}

pub fn alpha_bleeding(input: &str, output: &str) -> ImageResult<()> {
    let mut image = image::open(input)?.into_rgba8();
    let mut bleeder = AlphaBleeder::default();
    bleeder.execute(&mut image);
    image.save(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_alpha_bleeding() {
        let original = "tests/original.png";
        let output = "tests/alpha_bleeding_using_rs.png";
        let compared = "tests/alpha_bleeding_using_d.png";
        let result = alpha_bleeding(original, output);
        assert!(result.is_ok());
        let output = image::open(output).unwrap().into_rgba8();
        let expected = image::open(compared).unwrap().into_rgba8();
        assert_eq!(output, expected);
    }
}
