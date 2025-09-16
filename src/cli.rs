use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum QrSize {
    Small,
    Medium,
    Large,
}

#[derive(Parser, Debug)]
#[command(name = "qrt")]
#[command(about = "Generate QR codes in terminal")]
pub struct Args {
    #[arg(help = "URL or text to encode")]
    pub input: String,

    #[arg(long, short, default_value = "medium", help = "Size of the QR code")]
    pub size: QrSize,
}

impl QrSize {
    pub fn block_size(&self) -> usize {
        match self {
            QrSize::Small => 1,
            QrSize::Medium => 1,
            QrSize::Large => 2,
        }
    }
}