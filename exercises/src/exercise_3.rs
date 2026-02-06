use photon::PhotonImage;

#[allow(unused)]
const BYTES_PER_PIXEL: u32 = 4; // red, green, blue, alpha

/// Stitches two images together horizontally.
pub fn transform(left: &PhotonImage, right: &PhotonImage) -> PhotonImage {
    assert!(left.get_width() > 0);
    assert!(right.get_width() > 0);

    let _new_width = left.get_width() + right.get_width();
    let _new_height = left.get_height().max(right.get_height());

    let mut pixels = vec![255u8; (new_width * new_height * BYTES_PER_PIXEL) as usize];

    // Copy left image to the start.
    copy_into(&mut pixels, new_width, &left, 0);

    // Copy right image to where the left image ends.
    copy_into(&mut pixels, new_width, &right, left.get_width());

    PhotonImage::new(pixels, new_width, new_height)
}

/// Copies an image into a pixel buffer at a given x offset
fn copy_into(dst: &mut [u8], dst_width: u32, src: &PhotonImage, x_offset: u32) {
    let src_pixels = src.get_raw_pixels();
    let src_width = src.get_width();
    let src_height = src.get_height();

    for y in 0..src_height {
        for x in 0..src_width {
            // we calculated the index of our pixel in the source image pixel buffer `src_pixels`
            let src_index = ((y * src_width + x) * BYTES_PER_PIXEL) as usize;
            let dst_index = ((y * dst_width + x_offset + x) * 4) as usize;

            dst[dst_index] = src_pixels[src_index];
            dst[dst_index + 1] = src_pixels[src_index + 1];
            dst[dst_index + 2] = src_pixels[src_index + 2];
            dst[dst_index + 3] = src_pixels[src_index + 3];
        }
    }
}
