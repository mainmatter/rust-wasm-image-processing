use photon::PhotonImage;

const BYTES_PER_PIXEL: u32 = 4; // red, green, blue, alpha

/// Stitches two images together horizontally.
pub fn transform(left: &PhotonImage, right: &PhotonImage) -> PhotonImage {
    assert!(left.get_width() > 0);
    assert!(right.get_width() > 0);

    let new_width = left.get_width() + right.get_width();
    let new_height = left.get_height().max(right.get_height());

    // TODO initialize a vec that has space for new new_width * new_height pixels
    // TODO Copy left image to the start.
    // TODO Copy right image to where the left image ends.

    // PhotonImage::new(pixels, new_width, new_height)
    todo!()
}

/// Copies an image into a pixel buffer at a given x offset
fn _copy_into(_dst: &mut [u8], _dst_width: u32, src: &PhotonImage, _x_offset: u32) {
    let _src_pixels = src.get_raw_pixels();
    let src_width = src.get_width();
    let src_height = src.get_height();

    for y in 0..src_height {
        for x in 0..src_width {
            // we calculated the index of our pixel in the source image pixel buffer `src_pixels`
            let _src_index = ((y * src_width + x) * BYTES_PER_PIXEL) as usize;

            // we need to compute the index of our pixel in the destination image
            // and then copy the pixels 4 bytes into the destination position
        }
    }
}
