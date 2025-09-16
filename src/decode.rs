use std::path::Path;

pub fn decode_qr_from_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    if !Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path).into());
    }

    let img = image::open(file_path)?;
    let gray_img = img.to_luma8();
    let mut img_data = rqrr::PreparedImage::prepare(gray_img);
    let grids = img_data.detect_grids();
    
    if grids.is_empty() {
        return Err("No QR code detected".into());
    }
    
    let (_, content) = grids[0].decode()?;
    Ok(content)
}