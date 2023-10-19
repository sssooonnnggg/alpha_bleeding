use alpha_bleeding::*;
use image::ImageResult;

fn main() -> ImageResult<()> {
    alpha_bleeding("original.png", "alpha-bleeding.png")
}
