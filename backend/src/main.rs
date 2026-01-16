use axum::{Router, extract::Query, http::header, response::IntoResponse, routing::get};
use photon::PhotonImage;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/exercise_1", get(exercise_1))
        .route("/api/exercise_2", get(exercise_2));

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
        transformers::exercise_1::transform(photon_image);
    })
    .await
}

#[derive(Deserialize)]
struct Exercise2Params {
    filter: String,
}

async fn exercise_2(
    Query(ImageUrl { image_url }): Query<ImageUrl>,
    Query(Exercise2Params { filter }): Query<Exercise2Params>,
) -> impl IntoResponse {
    println!("Called exercise_2 with: {image_url}, {filter}");
    process_image(&image_url, move |photon_image| {
        transformers::exercise_2::transform(photon_image, &filter);
    })
    .await
}

async fn process_image<F: FnOnce(&mut PhotonImage)>(
    image_url: &str,
    f: F,
) -> impl IntoResponse + use<F> {
    let response = reqwest::get(image_url).await.unwrap();
    let body = response.bytes().await.unwrap();
    let mut photon_image = photon::native::open_image_from_bytes(&body).unwrap();

    f(&mut photon_image);

    let output = photon_image.get_bytes_jpeg(80);
    ([(header::CONTENT_TYPE, "image/jpeg")], output)
}
