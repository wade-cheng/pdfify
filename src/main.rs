use clap::Parser;
use image::io::Reader as ImageReader;
use std::process::Command;
use std::fs;

/// A program that takes image files and combines them into a pdf using imagemagick's `convert`. Supported image types depend on https://crates.io/crates/image#supported-image-formats
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// image files to pdfify
    #[arg(short, long, num_args(0..), required=true)]
    files: Option<Vec<String>>,

    /// output file name
    #[arg(short, long)]
    output: Option<String>,

    /// show verbose output
    #[arg(short, long)]
    verbose: bool,

    /// delete both the processed and original images after pdfifying
    #[arg(short, long="dimages")]
    delete_images: bool,

    /// delete only the original images after pdfifying
    #[arg(long="doriginal")]
    delete_original: bool,

    /// delete only the processed images after pdfifying
    #[arg(long="dprocessed")]
    delete_processed: bool,

    /// adjust brightness by a custom amount (integer amount)
    #[arg(short, long)]
    brighten: Option<i32>,

    /// adjust contrast by a custom amount (-100.0 to 100.0 percent)
    #[arg(short, long)]
    contrast: Option<f32>,
}

const DEFAULT_OUTPUT_NAME: &str = "pdfified.pdf";
const DEFAULT_BRIGHTEN: i32 = 80;
const DEFAULT_CONTRAST: f32 = 40.0;

/// processes given image paths and saves the page numbers
fn process_images(cli: &Cli) {
    let file_names: Vec<String> = cli.files.clone().expect("Something failed.");
    let verbose: bool = cli.verbose.clone();

    let contrast_value = cli.contrast.clone().unwrap_or(DEFAULT_CONTRAST);
    let brighten_value = cli.brighten.clone().unwrap_or(DEFAULT_BRIGHTEN);

    if verbose {
        println!("using contrast={contrast_value} and brighten={brighten_value}");
    }

    for (i, file_name) in file_names.iter().enumerate() {
        let i = i + 1;
        if verbose {
            println!("processing {}", file_name);
        }

        let img = ImageReader::open(file_name)
            .expect("Could not open the file.")
            .decode()
            .expect("Could not decode the file.");

        // this doesn't aCtually make processing speeds faster
        // i assume a lot of time is spent on just reading large inputs
        // if img.width() * img.height() > 1000 * 1000 {
        //     img = img.resize(1000, 1000, FilterType::Gaussian);
        // }
    
        let img = img.adjust_contrast(contrast_value);
        let img = img.brighten(brighten_value);
        let img = img.grayscale();

        img.save(format!("p{i}_{file_name}")).unwrap();
    }
}

/// takes original image paths and combines the processed versions into a pdf.
/// `file_names` should be identical to the one given to `process_images()`.
fn save_pdf(cli: &Cli) {
    let file_names: Vec<String> = cli.files.clone().expect("Something failed.");
    let verbose: bool = cli.verbose.clone();

    let mut convert = Command::new("convert");
    
    for (i, file_name) in file_names.iter().enumerate() {
        let i = i + 1;
        convert.arg(format!("p{i}_{file_name}"));
    }
    
    let output_name = cli.output.clone().unwrap_or(DEFAULT_OUTPUT_NAME.to_string());
    convert.arg(&output_name);

    if verbose {
        println!("trying to save to {output_name}");
    }
    convert.output().unwrap();

    println!("pdf file saved at {output_name}");
}

fn delete_all_images(cli: &Cli) {
    delete_original_images(&cli);
    delete_processed_images(&cli);
}

fn delete_original_images(cli: &Cli) {
    let file_names: Vec<String> = cli.files.clone().expect("Something failed.");

    for file_name in file_names {
        fs::remove_file(&file_name)
            .expect(&format!{"Removing file {file_name} failed."});
    }
}

fn delete_processed_images(cli: &Cli) {
    let file_names: Vec<String> = cli.files.clone().expect("Something failed.");

    for (i, file_name) in file_names.iter().enumerate() {
        let i = i + 1;
        fs::remove_file(format!("p{i}_{file_name}"))
            .expect(&format!{"Removing file p{i}_{file_name} failed."});
    }
}

fn image_deletion(cli: &Cli) {
    let dimages = cli.delete_images.clone();
    let doriginal = cli.delete_original.clone();
    let dprocessed = cli.delete_processed.clone();

    if dimages {
        delete_all_images(cli);
        return;
    } 
    
    if doriginal {
        delete_original_images(cli);
    }

    if dprocessed {
        delete_processed_images(cli);
    }
}

fn main() {
    let cli = Cli::parse();

    process_images(&cli);

    save_pdf(&cli);

    image_deletion(&cli);
}