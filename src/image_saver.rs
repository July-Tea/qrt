use image::{DynamicImage, ImageBuffer, Rgb};
use qrcode::QrCode;
use std::io::{self, Write};
use std::path::Path;

pub fn save_qr_image(
    qr_code: &QrCode,
    filename_option: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let filename = match filename_option {
        Some(name) => {
            if name.ends_with(".png") {
                name.to_string()
            } else {
                format!("{}.png", name)
            }
        }
        None => "output.png".to_string(),
    };

    if Path::new(&filename).exists() {
        print!("File '{}' already exists. Overwrite? (y/n): ", filename);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => {}
            _ => return Err("Operation cancelled".into()),
        }
    }

    // Create QR code image
    let image = create_qr_image(qr_code);

    // Save as PNG in current directory
    image.save_with_format(&filename, image::ImageFormat::Png)?;

    Ok(filename)
}

fn create_qr_image(qr_code: &QrCode) -> DynamicImage {
    let size = qr_code.width();
    let scale = 10; // Each QR module becomes 10x10 pixels
    let image_size = size * scale;

    let mut image = ImageBuffer::new(image_size as u32, image_size as u32);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let qr_x = x as usize / scale;
        let qr_y = y as usize / scale;

        let color = if qr_code[(qr_x, qr_y)] == qrcode::Color::Dark {
            Rgb([0, 0, 0]) // Black for dark modules
        } else {
            Rgb([255, 255, 255]) // White for light modules
        };

        *pixel = color;
    }

    DynamicImage::ImageRgb8(image)
}
