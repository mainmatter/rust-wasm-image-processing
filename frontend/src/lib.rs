use std::panic;
use wasm_bindgen::prelude::*;

// pub use wasm_bindgen_rayon::init_thread_pool;

// #[wasm_bindgen(start)]
#[wasm_bindgen]
pub fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    // console_log::init().unwrap();
}

#[wasm_bindgen]
pub fn exercise_1(image_data: &[u8]) -> Vec<u8> {
    let photon_image = photon::native::open_image_from_bytes(image_data).unwrap();
    let output_image = exercises::exercise_1::transform(photon_image);
    output_image.get_bytes_jpeg(80)
}

#[wasm_bindgen]
pub fn exercise_2(image_data: &[u8], filter: &str) -> Vec<u8> {
    let photon_image = photon::native::open_image_from_bytes(image_data).unwrap();
    let output_image = exercises::exercise_2::transform(photon_image, filter);
    output_image.get_bytes_jpeg(80)
}

#[wasm_bindgen]
pub fn exercise_3(left_image: &[u8], right_image: &[u8]) -> Vec<u8> {
    let left_image = photon::native::open_image_from_bytes(left_image).unwrap();
    let right_image = photon::native::open_image_from_bytes(right_image).unwrap();

    // Create thumbnail strip with these widths
    let output_image = exercises::exercise_3::transform(left_image, right_image);

    output_image.get_bytes_jpeg(80)
}

#[wasm_bindgen]
pub fn exercise_4(image_data: &[u8], widths: &[u32]) -> Vec<u8> {
    let photon_image = photon::native::open_image_from_bytes(image_data).unwrap();

    let output_image = exercises::exercise_4::transform(photon_image, &widths);

    output_image.get_bytes_jpeg(80)
}
