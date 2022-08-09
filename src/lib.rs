// QR Decoder module.

mod console;

use image::DynamicImage;
use rqrr::PreparedImage;

wit_bindgen_rust::export!("./qrdecoder_module.wit");

use console::Console;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
            let mut image = PreparedImage::prepare(gray);

            let grids = image.detect_grids();
            for grid in grids {
                if let Ok((_meta, result)) = grid.decode() {
                    return Some(result);
                }
            }
            None
        } else {
            Console::error("Failed to create RgbaImage");
            None
        }
    }
}
