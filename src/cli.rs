use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum QrSize {
    Small,
    Large,
}

#[derive(Parser, Debug)]
#[command(name = "qrt")]
#[command(about = "qrt (QR Tools) - Generate or decode QR codes")]
#[command(version)]
#[command(arg_required_else_help = true)]
pub struct Args {
    #[arg(help = "Text to encode OR image path to decode")]
    pub input: String,

    #[arg(long, short, default_value = "small", help = "Size of the QR code")]
    pub size: QrSize,

    #[arg(
        long,
        short = 'S',
        help = "Save QR code as PNG image. Use --save for 'output.png' or --save filename"
    )]
    pub save: Option<Option<String>>,

    #[arg(long, short, help = "Decode QR code from image", conflicts_with_all = ["size", "save"])]
    pub decode: bool,
}
