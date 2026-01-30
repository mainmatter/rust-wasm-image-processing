use photon::PhotonImage;

// 02 show off control flow & match => switch between filters a number of filters based on a string param (panic on unknown)
pub fn transform(mut img: PhotonImage, filter: &str) -> PhotonImage {
    match filter {
        "cali" => photon::filters::cali(&mut img),
        "dramatic" => photon::filters::dramatic(&mut img),
        "duotone_horizon" => photon::filters::duotone_horizon(&mut img),
        "duotone_lilac" => photon::filters::duotone_lilac(&mut img),
        "golden" => photon::filters::golden(&mut img),
        "lofi" => photon::filters::lofi(&mut img),
        "pastel_pink" => photon::filters::pastel_pink(&mut img),
        _ => panic!("no such filter"),
    }
    img
}
