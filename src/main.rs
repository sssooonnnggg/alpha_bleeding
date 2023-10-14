use alpha_bleeding::perform_alpha_bleeding_aux;
use image::ImageResult;
mod alpha_bleeding;

fn main() -> ImageResult<()> {
    perform_alpha_bleeding_aux("original.png", "alpha-bleeding.png")
}
