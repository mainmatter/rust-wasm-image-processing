use std::panic;
use wasm_bindgen::prelude::*;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn exercise_1(image_data: &[u8]) -> Vec<u8> {
    let mut photon_image = photon::native::open_image_from_bytes(image_data).unwrap();
    transformers::exercise_1::transform(&mut photon_image);
    photon_image.get_bytes_jpeg(80)
}

#[wasm_bindgen]
pub fn exercise_2(image_data: &[u8], do_one_thing: bool, value: f32) -> Vec<u8> {
    let mut photon_image = photon::native::open_image_from_bytes(image_data).unwrap();
    transformers::exercise_2::transform(&mut photon_image, do_one_thing, value);
    photon_image.get_bytes_jpeg(80)
}
