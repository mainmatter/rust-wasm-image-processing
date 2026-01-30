use photon::PhotonImage;
use photon::transform::{SamplingFilter, resize};

/// Creates a thumbnail strip by resizing the image to multiple widths
/// and stitching them together horizontally.
pub fn transform(img: PhotonImage, widths: &[u32]) -> PhotonImage {
    // First, resize the image for each width in the slice
    let mut thumbnails = Vec::new();

    for width in widths {
        // Calculate height to maintain aspect ratio
        let aspect_ratio = img.get_height() as f32 / img.get_width() as f32;
        let height = (*width as f32 * aspect_ratio) as u32;

        // Resize and collect
        let thumbnail = resize(&img, *width, height, SamplingFilter::Nearest);
        thumbnails.push(thumbnail);
    }

    // Calculate dimensions for the final composite image
    let total_width: u32 = thumbnails.iter().map(|t| t.get_width()).sum();
    let max_height: u32 = thumbnails.iter().map(|t| t.get_height()).max().unwrap_or(0);

    // Create a new pixel buffer (RGBA = 4 bytes per pixel)
    let mut composite_pixels = vec![255u8; (total_width * max_height * 4) as usize];

    // Copy each thumbnail into the composite at the right position
    let mut x_offset: u32 = 0;

    for thumbnail in &thumbnails {
        let thumb_pixels = thumbnail.get_raw_pixels();
        let thumb_width = thumbnail.get_width();
        let thumb_height = thumbnail.get_height();

        // Copy row by row
        for y in 0..thumb_height {
            for x in 0..thumb_width {
                let src_idx = ((y * thumb_width + x) * 4) as usize;
                let dst_idx = ((y * total_width + x_offset + x) * 4) as usize;

                // Copy RGBA
                composite_pixels[dst_idx] = thumb_pixels[src_idx];
                composite_pixels[dst_idx + 1] = thumb_pixels[src_idx + 1];
                composite_pixels[dst_idx + 2] = thumb_pixels[src_idx + 2];
                composite_pixels[dst_idx + 3] = thumb_pixels[src_idx + 3];
            }
        }

        x_offset += thumb_width;
    }

    PhotonImage::new(composite_pixels, total_width, max_height)
}
