use crate::cli::QrSize;
use qrcode::{Color, QrCode};

pub fn display_qr(qr_code: &QrCode, size: &QrSize) {
    let block_size = size.block_size();
    let width = qr_code.width();

    for row in 0..width {
        for _ in 0..block_size {
            for col in 0..width {
                let is_dark = qr_code[(col, row)] == Color::Dark;
                let block = if is_dark { "██" } else { "  " };
                for _ in 0..block_size {
                    print!("{}", block);
                }
            }
            println!();
        }
    }
}
