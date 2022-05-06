use clap::Parser;

// Print image file meta-data option
#[derive(Parser)]
#[clap(
name = "iftat",
author = "Nakaue Yoshito",
version = "0.1.0",
about = "Command line tool to convert image file format, get meta-data, etc..."
)]
struct Options {
    #[clap(short = 'd', long = "data", help = "Print image file meta data (Default)")]
    data_flag: bool,

    #[clap(short = 'j', long = "jpeg", help = "Convert input image file to jpeg file")]
    jpeg_flag: bool,

    #[clap(short = 'p', long = "png", help = "Convert input image file to png file")]
    png_flag: bool,

    // Input image files
    #[clap(required = true)]
    file: Vec<String>,
}

fn main() {
    let options: Options = Options::parse();

    for f in options.file {
        println!("{:?}", f);
    }
}

#[allow(dead_code)]
fn hello(name: Option<String>) -> String {
    return format!("Hello, {}", if let Some(n) = name {
        n
    } else {
        "World!".to_string()
    })
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
