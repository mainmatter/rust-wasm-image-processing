use photon::PhotonImage;

pub fn transform(mut img: PhotonImage, do_one_thing: bool, value: f32) -> PhotonImage {
    if do_one_thing {
        photon::effects::adjust_brightness(&mut img, value as i16);
    } else {
        photon::effects::adjust_contrast(&mut img, value);
    }
    img
}
