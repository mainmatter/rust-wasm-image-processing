use photon::PhotonImage;

pub fn transform(img: &mut PhotonImage) {
    photon::filters::dramatic(img);
}
