use axum::{Router, extract::Query, http::header, response::IntoResponse, routing::get};
use photon::PhotonImage;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/exercise_1", get(exercise_1))
        .route("/api/exercise_2", get(exercise_2))
        .route("/api/exercise_3", get(exercise_3))
        .route("/api/exercise_4", get(exercise_4));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct ImageUrl {
    image_url: String,
}

async fn exercise_1(Query(ImageUrl { image_url }): Query<ImageUrl>) -> impl IntoResponse {
    println!("Called exercise_1 with: {image_url}");
    process_image(&image_url, move |photon_image| {
        transformers::exercise_1::transform(photon_image)
    })
    .await
}

#[derive(Deserialize)]
struct Exercise2Params {
    do_one_thing: bool,
    value: f32,
}

async fn exercise_2(
    Query(ImageUrl { image_url }): Query<ImageUrl>,
    Query(Exercise2Params {
        do_one_thing,
        value,
    }): Query<Exercise2Params>,
) -> impl IntoResponse {
    println!("Called exercise_2 with: {image_url}, {do_one_thing}, {value}");
    process_image(&image_url, move |photon_image| {
        transformers::exercise_2::transform(photon_image, do_one_thing, value)
    })
    .await
}

async fn exercise_3(Query(ImageUrl { image_url }): Query<ImageUrl>) -> impl IntoResponse {
    println!("Called exercise_3 with: {image_url}");
    process_image(&image_url, move |photon_image| {
        let widths = [50, 100, 200, 400, 800, 1600];
        transformers::exercise_3::transform(photon_image, &widths)
    })
    .await
}

async fn exercise_4(Query(ImageUrl { image_url }): Query<ImageUrl>) -> impl IntoResponse {
    println!("Called exercise_4 with: {image_url}");
    process_image(&image_url, move |photon_image| {
        let widths = [50, 100, 200, 400, 800, 1600];
        transformers::exercise_4::transform(photon_image, &widths)
    })
    .await
}

async fn process_image<F>(image_url: &str, f: F) -> impl IntoResponse + use<F>
where
    F: FnOnce(PhotonImage) -> PhotonImage,
{
    let response = reqwest::get(image_url).await.unwrap();
    let body = response.bytes().await.unwrap();
    let photon_image = photon::native::open_image_from_bytes(&body).unwrap();

    let output_image = f(photon_image);

    let output = output_image.get_bytes_jpeg(80);
    ([(header::CONTENT_TYPE, "image/jpeg")], output)
}
