use std::panic;
use wasm_bindgen::prelude::*;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().unwrap();
}

#[wasm_bindgen]
pub fn exercise_1(image_data: &[u8]) -> Vec<u8> {
    let photon_image = photon::native::open_image_from_bytes(image_data).unwrap();
    let output_image = transformers::exercise_1::transform(photon_image);
    output_image.get_bytes_jpeg(80)
}

#[wasm_bindgen]
pub fn exercise_2(image_data: &[u8], do_one_thing: bool, value: f32) -> Vec<u8> {
    let photon_image = photon::native::open_image_from_bytes(image_data).unwrap();
    let output_image = transformers::exercise_2::transform(photon_image, do_one_thing, value);
    output_image.get_bytes_jpeg(80)
}

#[wasm_bindgen]
pub fn exercise_3(image_data: &[u8]) -> Vec<u8> {
    let photon_image = photon::native::open_image_from_bytes(image_data).unwrap();

    // Create thumbnail strip with these widths
    let widths = [50, 100, 200, 400, 800, 1600];
    let output_image = transformers::exercise_3::transform(photon_image, &widths);

    output_image.get_bytes_jpeg(80)
}

#[wasm_bindgen]
pub fn exercise_4(image_data: &[u8]) -> Vec<u8> {
    let photon_image = photon::native::open_image_from_bytes(image_data).unwrap();

    let widths = [50, 100, 200, 400, 800, 1600];
    let output_image = transformers::exercise_4::transform(photon_image, &widths);

    output_image.get_bytes_jpeg(80)
}

pub use wasm_bindgen_rayon::init_thread_pool;
