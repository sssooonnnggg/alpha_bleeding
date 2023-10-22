use alpha_bleeding::*;
use image::ImageResult;

use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    /// Input image path to be processed
    input: Option<String>,
    /// Output image path, if not provided, replace the input image
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
