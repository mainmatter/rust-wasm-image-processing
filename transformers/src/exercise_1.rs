use photon::PhotonImage;

pub fn transform(mut img: PhotonImage) -> PhotonImage {
    photon::filters::dramatic(&mut img);
    img
}
