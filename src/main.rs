mod cli;
mod display;
mod image_saver;
mod qr;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    match qr::generate_qr(&args.input) {
        Ok(qr_code) => {
            // Always display in terminal
            display::display_qr(&qr_code, &args.size);

            // Save to file if requested
            if let Some(save_option) = &args.save {
                let filename = save_option.as_deref();
                match image_saver::save_qr_image(&qr_code, filename) {
                    Ok(filename) => {
                        println!("Saved: {}", filename);
                    }
                    Err(e) => {
                        eprintln!("Error saving image: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error generating QR code: {}", e);
            std::process::exit(1);
        }
    }
}
