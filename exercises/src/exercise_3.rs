use photon::PhotonImage;

/// Stitches two image together horizontally.
pub fn transform(left: PhotonImage, right: PhotonImage) -> PhotonImage {
    assert!(left.get_width() > 0);
    assert!(right.get_width() > 0);

    let new_width = left.get_width() + right.get_width();
    let new_height = left.get_height().max(right.get_height());

    let mut pixels = vec![255u8; (new_width * new_height * 4) as usize];

    // Copy left image
    copy_into(&mut pixels, new_width, &left, 0);

    // Copy right image
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
            let src_idx = ((y * src_width + x) * 4) as usize;
            let dst_idx = ((y * dst_width + x_offset + x) * 4) as usize;

            dst[dst_idx] = src_pixels[src_idx];
            dst[dst_idx + 1] = src_pixels[src_idx + 1];
            dst[dst_idx + 2] = src_pixels[src_idx + 2];
            dst[dst_idx + 3] = src_pixels[src_idx + 3];
        }
    }
}
