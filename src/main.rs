mod cli;
mod qr;
mod display;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();
    
    match qr::generate_qr(&args.input) {
        Ok(matrix) => {
            display::display_qr(&matrix, &args.size);
        }
        Err(e) => {
            eprintln!("Error generating QR code: {}", e);
            std::process::exit(1);
        }
    }
}
