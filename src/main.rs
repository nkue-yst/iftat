use std::fs;
use std::path::PathBuf;

use chrono::prelude::{DateTime, Utc, Datelike};
use clap::Parser;
use image::{GenericImageView, ImageFormat};

mod cli;

fn print_data(file: PathBuf) {
    // Get file metadata
    let metadata = fs::metadata(file.to_str().unwrap()).unwrap();

    // Print file name and format
    println!("File   : {}", file.to_str().unwrap());
    println!("Format : {}", file.extension().unwrap().to_str().unwrap());

    // Load image file
    let img = image::open(file.to_str().unwrap()).unwrap();

    // Print image size
    println!("Width  : {}", img.dimensions().0);
    println!("Height : {}", img.dimensions().1);

    // Print data size
    println!("Size   : {} bytes", metadata.len());

    // Print modified date
    let modified_date: DateTime<Utc> = metadata.modified().unwrap().into();
    let modified_date = format!("{}/{}/{}", modified_date.year(), modified_date.month(), modified_date.day());
    println!("Updated: {}", modified_date);
}

fn convert_to_jpeg(file: PathBuf) {
    // Load original image file
    let img = image::open(file.to_str().unwrap()).unwrap();

    // Save as jpeg file
    let out_path = file.to_str().unwrap();
    img.save_with_format(out_path.replace(".png", ".jpeg"), ImageFormat::Jpeg).unwrap();
}

fn convert_to_png(file: PathBuf) {
    // Load original image file
    let img = image::open(file.to_str().unwrap()).unwrap();

    // Save as jpeg file
    let out_path = file.to_str().unwrap();
    img.save_with_format(out_path.replace(".jpeg", ".png"), ImageFormat::Jpeg).unwrap();
}

fn main() {
    let options = cli::Options::parse();

    if options.data_flag {
        print_data(PathBuf::from(&options.file[0]).to_path_buf());
    }

    if options.jpeg_flag {
        convert_to_jpeg(PathBuf::from(&options.file[0]).to_path_buf());
    }

    if options.png_flag {
        convert_to_png(PathBuf::from(&options.file[0]).to_path_buf());
    }
}

#[allow(dead_code)]
fn hello(name: Option<String>) -> String {
    return format!(
        "Hello, {}",
        if let Some(n) = name {
            n
        } else {
            "World!".to_string()
        }
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!("Hello, World!", hello(None));
        assert_eq!("Hello, nkue-yst", hello(Some("nkue-yst".to_string())));
    }
}
