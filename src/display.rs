use crate::qr::QrMatrix;
use crate::cli::QrSize;

pub fn display_qr(matrix: &QrMatrix, size: &QrSize) {
    let block_size = size.block_size();
    
    for row in &matrix.data {
        for _ in 0..block_size {
            for &pixel in row {
                let block = if pixel { "██" } else { "  " };
                for _ in 0..block_size {
                    print!("{}", block);
                }
            }
            println!();
        }
    }
}