use photon::{PhotonImage, transform::{SamplingFilter, resize}};

/// Welcome to the Wasm Image Transformation Workshop!
///
/// During our short time together you will see a bunch of cool things that Rust & WebAssembly can do!
///
/// This workshop is divided into four exercises, each one a bit more complicated than the previous.
/// During these exercises we will be building **image transformers** that accept an input image, apply
/// some transformation and return an output image.
pub fn transform(img: &PhotonImage, target_width: u32) -> PhotonImage {
    // Calculate aspect ratio
    let aspect_ratio = img.get_width() as f32 / img.get_height() as f32;
    let target_height = (target_width as f32 / aspect_ratio) as u32;

    // we could use `return img;` but Rust automatically returns the last block's value in a function. How nice!
    resize(&img, target_width, target_height, SamplingFilter::Nearest)
}
