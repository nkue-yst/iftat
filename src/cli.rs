use clap::Parser;

// Print image file meta-data option
#[derive(Parser)]
#[clap(
    name = "iftat",
    author = "Nakaue Yoshito",
    version = "0.1.0",
    about = "Command line tool to convert image file format, get meta-data, etc..."
)]
pub struct Options {
    #[clap(
        short = 'd',
        long = "data",
        help = "Print image file meta data (Default)"
    )]
    pub data_flag: bool,

    #[clap(
        short = 'j',
        long = "jpeg",
        help = "Convert input image file to jpeg file"
    )]
    pub jpeg_flag: bool,

    #[clap(
        short = 'p',
        long = "png",
        help = "Convert input image file to png file"
    )]
    pub png_flag: bool,

    // Input image files
    #[clap(required = true)]
    pub file: Vec<String>,
}
