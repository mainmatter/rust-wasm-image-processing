use photon::PhotonImage;

/// A transformation that applies a color filter. The filter is determined by the `filter` argument.
pub fn transform(img: &mut PhotonImage, filter: &str) {
    match filter {
        "cali" => photon::filters::cali(img),
        "dramatic" => photon::filters::dramatic(img),
        "duotone_horizon" => photon::filters::duotone_horizon(img),
        "duotone_lilac" => photon::filters::duotone_lilac(img),
        "golden" => photon::filters::golden(img),
        "lofi" => photon::filters::lofi(img),
        "pastel_pink" => photon::filters::pastel_pink(img),
        _ => panic!("no such filter: {filter}"),
    }
}
