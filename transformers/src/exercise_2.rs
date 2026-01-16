use photon::PhotonImage;

pub fn transform(img: &mut PhotonImage, do_one_thing: bool, value: f32) {
    if do_one_thing {
        photon::effects::adjust_brightness(img, value as i16);
    } else {
        photon::effects::adjust_contrast(img, value);
    }
}
