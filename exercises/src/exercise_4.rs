use photon::PhotonImage;
use photon::transform::{SamplingFilter, resize};
use rayon::prelude::*;

/// Creates a thumbnail strip with a color filter applied, using parallel iteration.
///
/// This exercise teaches:
/// - Iterator combinators (`.iter()`, `.map()`)
/// - Parallel iteration with rayon (`.par_iter()`, `.reduce()`)
/// - Processing pipelines
pub fn transform(img: PhotonImage, widths: &[u32]) -> PhotonImage {
    // Create a parallel iterator pipeline:
    // 1. Iterate over widths in parallel
    // 2. Map each width to a resized + filtered thumbnail
    // 3. Reduce into a single stitched image
    widths
        .par_iter()
        .map(|width| {
            // Calculate height to maintain aspect ratio
            let aspect_ratio = img.get_height() as f32 / img.get_width() as f32;
            let height = (*width as f32 * aspect_ratio) as u32;

            // Resize the image
            resize(&img, *width, height, SamplingFilter::Nearest)
        })
        .reduce_with(|left, right| {
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
        })
        .unwrap()
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
