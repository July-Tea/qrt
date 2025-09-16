use qrcode::{Color, QrCode};

pub struct QrMatrix {
    pub data: Vec<Vec<bool>>,
}

pub fn generate_qr(input: &str) -> Result<QrMatrix, Box<dyn std::error::Error>> {
    let code = QrCode::new(input)?;
    let matrix_data = code.to_colors();
    let width = (matrix_data.len() as f64).sqrt() as usize;

    let mut data = vec![vec![false; width]; width];

    for (i, &bit) in matrix_data.iter().enumerate() {
        let row = i / width;
        let col = i % width;
        data[row][col] = bit == Color::Dark;
    }

    Ok(QrMatrix { data })
}
