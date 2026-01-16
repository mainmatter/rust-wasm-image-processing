use photon::PhotonImage;

// 02 show off control flow & match => switch between filters a number of filters based on a string param (panic on unknown)

pub fn transform(img: &mut PhotonImage, filter: &str) {
    match filter {
        "cali" => photon::filters::cali(img),
        "dramatic" => photon::filters::dramatic(img),
        "duotone_horizon" => photon::filters::duotone_horizon(img),
        "duotone_lilac" => photon::filters::duotone_lilac(img),
        "duotone_ochre" => photon::filters::duotone_ochre(img),
        "duotone_violette" => photon::filters::duotone_violette(img),
        "firenze" => photon::filters::firenze(img),
        "golden" => photon::filters::golden(img),
        "lix" => photon::filters::lix(img),
        "lofi" => photon::filters::lofi(img),
        "neue" => photon::filters::neue(img),
        "obsidian" => photon::filters::obsidian(img),
        "pastel_pink" => photon::filters::pastel_pink(img),
        "ryo" => photon::filters::ryo(img),
        _ => panic!("no such filter"),
    }
}
