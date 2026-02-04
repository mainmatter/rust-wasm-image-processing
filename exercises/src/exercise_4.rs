use photon::PhotonImage;
use rayon::prelude::*;

const BYTES_PER_PIXEL: u32 = 4; // red, green, blue, alpha

/// Creates a thumbnail strip with a color filter applied, using parallel iteration.
pub fn transform(img: &PhotonImage, widths: &[u32]) -> PhotonImage {
    // Create a parallel iterator pipeline:
    // 1. Iterate over widths in parallel
    // 2. Map each width to a resized + filtered thumbnail
    // 3. Reduce into a single stitched image
    todo!()
}
