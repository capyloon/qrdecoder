// QR Decoder module.

mod console;

use image::DynamicImage;
use quircs::Quirc;

wit_bindgen_guest_rust::export!("./qrdecoder_module.wit");

use console::Console;

pub struct QrdecoderModule;

impl qrdecoder_module::QrdecoderModule for QrdecoderModule {
    fn decode_qr(data: Vec<u8>, width: u32, height: u32) -> Option<String> {
        Console::log(&format!(
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
            Console::error("Failed to create RgbaImage");
            None
        }
    }
}
