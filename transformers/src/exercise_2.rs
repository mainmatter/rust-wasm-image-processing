use photon::PhotonImage;

// 02 show off control flow & match => switch between filters a number of filters based on a string param (panic on unknown)
pub fn transform(mut img: PhotonImage, filter: &str) -> PhotonImage {
    match filter {
        "cali" => photon::filters::cali(&mut img),
        "dramatic" => photon::filters::dramatic(&mut img),
        "duotone_horizon" => photon::filters::duotone_horizon(&mut img),
        "duotone_lilac" => photon::filters::duotone_lilac(&mut img),
        "duotone_ochre" => photon::filters::duotone_ochre(&mut img),
        "duotone_violette" => photon::filters::duotone_violette(&mut img),
        "firenze" => photon::filters::firenze(&mut img),
        "golden" => photon::filters::golden(&mut img),
        "lix" => photon::filters::lix(&mut img),
        "lofi" => photon::filters::lofi(&mut img),
        "neue" => photon::filters::neue(&mut img),
        "obsidian" => photon::filters::obsidian(&mut img),
        "pastel_pink" => photon::filters::pastel_pink(&mut img),
        "ryo" => photon::filters::ryo(&mut img),
        _ => panic!("no such filter"),
    }
    img
}
