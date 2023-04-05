// QR Decoder module.

#[macro_use]
mod qrdecoder;

use image::DynamicImage;
use quircs::Quirc;
use qrdecoder::{console, types::Level};

pub struct QuircDecoder;

impl qrdecoder::Qrdecoder for QuircDecoder {
    fn decode_qr(data: Vec<u8>, width: u32, height: u32) -> Option<String> {
        console::msg(Level::Debug, &format!(
            "Looking for QR code in image: {}x{} ({} bytes)",
            width,
            height,
            data.len()
        ));

        if let Some(source) = image::RgbaImage::from_vec(width, height, data) {
            let gray = DynamicImage::ImageRgba8(source).into_luma8();
            let mut decoder = Quirc::default();

            let codes = decoder.identify(gray.width() as usize, gray.height() as usize, &gray);

            for code in codes {
                if let Ok(code) = code {
                    if let Ok(decoded) = code.decode() {
                        return Some(String::from_utf8_lossy(&decoded.payload).to_string());
                    }
                }
            }
            None
        } else {
            console::msg(Level::Error, "Failed to create RgbaImage");
            None
        }
    }
}

export_qrdecoder!(QuircDecoder);
