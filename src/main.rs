use alpha_bleeding::*;
use image::ImageResult;

use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    /// The path of the image to be fixed.
    input: Option<String>,
    /// The path where the fixed image will be saved. 
    /// If an output path is not provided, the original input image will be replaced with the fixed one.
    ouptut: Option<String>,
}

fn main() -> ImageResult<()> {
    let cli = Cli::parse();
    if let Some(input) = cli.input {
        let output = cli.ouptut.unwrap_or(input.clone());
        alpha_bleeding(&input, &output)
    } else {
        Ok(())
    }
}
