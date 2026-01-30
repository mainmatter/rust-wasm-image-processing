use axum::{Router, extract::Query, http::header, response::IntoResponse, routing::get};
use photon::PhotonImage;
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/exercise_1", get(exercise_1))
        .route("/exercise_2", get(exercise_2))
        .route("/exercise_3", get(exercise_3))
        .route("/exercise_4", get(exercise_4))
        .layer(TraceLayer::new_for_http());

    // We must use 127.0.0.1 as hostname for this to work in a dev container.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[tracing::instrument]
async fn ping() -> impl IntoResponse {
    tracing::info!("Called ping");
    "Pong!"
}

#[derive(Deserialize)]
struct ImageUrl {
    image_url: String,
}

#[tracing::instrument]
async fn exercise_1(Query(ImageUrl { image_url }): Query<ImageUrl>) -> impl IntoResponse {
    tracing::info!("Called exercise_1 with: {image_url}");
    process_image(&image_url, move |photon_image| {
        transformers::exercise_1::transform(photon_image)
    })
    .await
}

#[derive(Deserialize)]
struct Exercise2Params {
    filter: String,
}

#[tracing::instrument]
async fn exercise_2(
    Query(ImageUrl { image_url }): Query<ImageUrl>,
    Query(Exercise2Params { filter }): Query<Exercise2Params>,
) -> impl IntoResponse {
    tracing::info!("Called exercise_2 with: {image_url}, {filter}");
    process_image(&image_url, move |photon_image| {
        transformers::exercise_2::transform(photon_image, &filter)
    })
    .await
}

#[tracing::instrument]
async fn exercise_3(Query(ImageUrl { image_url }): Query<ImageUrl>) -> impl IntoResponse {
    tracing::info!("Called exercise_3 with: {image_url}");
    process_image(&image_url, move |photon_image| {
        let widths = [50, 100, 200, 400, 800, 1600];
        transformers::exercise_3::transform(photon_image, &widths)
    })
    .await
}

#[tracing::instrument]
async fn exercise_4(Query(ImageUrl { image_url }): Query<ImageUrl>) -> impl IntoResponse {
    tracing::info!("Called exercise_4 with: {image_url}");
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
    (
        [
            (header::CONTENT_TYPE, "image/jpeg"),
            (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
            (header::ACCESS_CONTROL_ALLOW_METHODS, "GET"),
        ],
        output,
    )
}
