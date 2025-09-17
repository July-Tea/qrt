use crate::cli::QrSize;
use qrcode::{Color, QrCode};

pub fn display_qr(qr_code: &QrCode, size: &QrSize) {
    let width = qr_code.width();

    match size {
        QrSize::Small => {
            // Vertical compression: merge every 2 rows into 1
            for row in (0..width).step_by(2) {
                for col in 0..width {
                    let upper_dark = qr_code[(col, row)] == Color::Dark;
                    let lower_dark = if row + 1 < width {
                        qr_code[(col, row + 1)] == Color::Dark
                    } else {
                        false
                    };

                    let block_char = match (upper_dark, lower_dark) {
                        (true, true) => "█",
                        (true, false) => "▀",
                        (false, true) => "▄",
                        (false, false) => " ",
                    };
                    print!("{}", block_char);
                }
                println!();
            }
        }
        QrSize::Large => {
            // Original method with single character blocks
            for row in 0..width {
                for col in 0..width {
                    let is_dark = qr_code[(col, row)] == Color::Dark;
                    let block = if is_dark { "██" } else { "  " };
                    print!("{}", block);
                }
                println!();
            }
        }
    }
}
