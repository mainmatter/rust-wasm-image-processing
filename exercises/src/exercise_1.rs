use photon::PhotonImage;

/// Welcome to the Wasm Image Transformation Workshop!
///
/// During our short time together you will see a bunch of cool things that Rust & WebAssembly can do!
///
/// This workshop is divided into four exercises, each one a bit more complicated than the previous.
/// During these exercises we will be building **image transformers** that accept an input image, apply
/// some transformation and return an output image.
pub fn transform(img: &PhotonImage, width: u32) -> PhotonImage {
    // Calculate aspect ratio
    let aspect_ratio = img.get_height() as f32 / img.get_width() as f32;
    let height = (width as f32 * aspect_ratio) as u32;

    // import and call the resize function here!
    todo!("call the resize function!")
}
