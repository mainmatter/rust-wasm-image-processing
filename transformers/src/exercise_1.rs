use photon::PhotonImage;

/// Welcome to the Wasm Image Transformation Workshop!
///
/// During our short time together you will see a bunch of cool things that Rust & WebAssembly can do!
///
/// This workshop is divided into four exercises, each one a bit more complicated than the next.
/// During these exercises we will be building **image transformers** that accept an input image, apply
/// some transformation and return an output image.
pub fn transform(mut img: PhotonImage) -> PhotonImage {
    // `img` is passed by value but `flipv` requires a mutable reference as input
    photon::transform::flipv(&mut img);
    // we could use `return img;` but it is usually simpler to simply use the block return value by writing
    // the result *without* ;
    img
}
