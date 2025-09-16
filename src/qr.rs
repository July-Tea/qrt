use qrcode::QrCode;

pub fn generate_qr(input: &str) -> Result<QrCode, Box<dyn std::error::Error>> {
    let code = QrCode::new(input)?;
    Ok(code)
}
