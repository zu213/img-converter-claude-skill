use clap::Parser;
use image::ImageReader;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "png-to-webp")]
#[command(about = "Converts PNG images to WebP format")]
struct Args {
    /// Input PNG file path
    input: PathBuf,

    /// Output WebP file path (defaults to same name with .webp extension)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Encoding quality (0-100, default 80)
    #[arg(short, long, default_value_t = 80.0)]
    quality: f32,
}

fn convert(input: &Path, output: &Path, quality: f32) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(input)?.decode()?;
    let rgba = img.to_rgba8();

    let encoder = webp::Encoder::from_rgba(rgba.as_raw(), rgba.width(), rgba.height());
    let webp_data = encoder.encode(quality);

    fs::write(output, &*webp_data)?;

    let input_size = fs::metadata(input)?.len();
    let ratio = (webp_data.len() as f64 / input_size as f64) * 100.0;
    println!(
        "{} -> {} ({} bytes, {:.1}% of original)",
        input.display(),
        output.display(),
        webp_data.len(),
        ratio
    );

    Ok(())
}

fn main() {
    let args = Args::parse();

    if args.quality < 0.0 || args.quality > 100.0 {
        eprintln!("Error: quality must be between 0 and 100");
        std::process::exit(1);
    }

    let output = args.output.unwrap_or_else(|| args.input.with_extension("webp"));

    if let Err(e) = convert(&args.input, &output, args.quality) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
