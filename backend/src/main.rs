use axum::{
    Router,
    extract::Query,
    http::header,
    response::{Html, IntoResponse},
    routing::get,
};
use photon::PhotonImage;
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let app = Router::new()
        .route("/", get(root))
        .route("/exercise_1", get(exercise_1))
        .route("/exercise_2", get(exercise_2))
        .route("/exercise_3", get(exercise_3))
        .route("/exercise_4", get(exercise_4))
        .route("/image_proxy", get(image_proxy))
        .layer(TraceLayer::new_for_http());

    // We must use 0.0.0.0 as hostname for this to work in a dev container.
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 3001);

    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[tracing::instrument]
async fn root() -> impl IntoResponse {
    tracing::info!("Called root");

    Html(
        "This is the <i>backend</i> of rust-wasm-image-processing<br><br>To get started with the exercises, browse the <i>frontend</i> at <a href=\"http://0.0.0.0:3000\">http://0.0.0.0:3000</a>",
    )
}

#[derive(Deserialize)]
struct Exercise1Params {
    image_url: String,
    width: u32,
}

#[tracing::instrument]
async fn exercise_1(
    Query(Exercise1Params { image_url, width }): Query<Exercise1Params>,
) -> impl IntoResponse {
    tracing::info!("Called exercise_1 with: {image_url} {width}");

    let photon_image = fetch_image(&image_url).await;
    let output_image = exercises::exercise_1::transform(&photon_image, width);
    let output_bytes = output_image.get_bytes_jpeg(80);

    let headers = [
        (header::CONTENT_TYPE, "image/jpeg"),
        (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
        (header::ACCESS_CONTROL_ALLOW_METHODS, "GET"),
    ];

    (headers, output_bytes)
}

#[derive(Deserialize)]
struct Exercise2Params {
    image_url: String,
    filter: String,
}

#[tracing::instrument]
async fn exercise_2(
    Query(Exercise2Params { image_url, filter }): Query<Exercise2Params>,
) -> impl IntoResponse {
    tracing::info!("Called exercise_2 with: {image_url}, {filter}");

    let mut photon_image = fetch_image(&image_url).await;
    exercises::exercise_2::transform(&mut photon_image, &filter);
    let output_bytes = photon_image.get_bytes_jpeg(80);

    let headers = [
        (header::CONTENT_TYPE, "image/jpeg"),
        (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
        (header::ACCESS_CONTROL_ALLOW_METHODS, "GET"),
    ];

    (headers, output_bytes)
}

#[derive(Deserialize)]
struct Exercise3Params {
    left: String,
    right: String,
}

#[tracing::instrument]
async fn exercise_3(
    Query(Exercise3Params { left, right }): Query<Exercise3Params>,
) -> impl IntoResponse {
    tracing::info!("Called exercise_3 with: {left} {right}");

    let (left, right) = tokio::join!(fetch_image(&left), fetch_image(&right));
    let output_image = exercises::exercise_3::transform(&left, &right);
    let output_bytes = output_image.get_bytes_jpeg(80);

    let headers = [
        (header::CONTENT_TYPE, "image/jpeg"),
        (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
        (header::ACCESS_CONTROL_ALLOW_METHODS, "GET"),
    ];

    (headers, output_bytes)
}

#[derive(Deserialize)]
struct Exercise4Params {
    image_url: String,
    width: Vec<u32>,
}

#[tracing::instrument]
async fn exercise_4(
    axum_extra::extract::Query(Exercise4Params { image_url, width }): axum_extra::extract::Query<
        Exercise4Params,
    >,
) -> impl IntoResponse {
    tracing::info!("Called exercise_4 with: {image_url} {width:?}");

    let photon_image = fetch_image(&image_url).await;
    let output_image = exercises::exercise_4::transform(&photon_image, &width);
    let output_bytes = output_image.get_bytes_jpeg(80);

    let headers = [
        (header::CONTENT_TYPE, "image/jpeg"),
        (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
        (header::ACCESS_CONTROL_ALLOW_METHODS, "GET"),
    ];

    (headers, output_bytes)
}

#[derive(Deserialize)]
struct ImageProxyParams {
    url: String,
}

// A proxy endpoint used in `workshop.js` for fetching images
// working around browser CORS restrictions.
#[tracing::instrument]
async fn image_proxy(
    Query(ImageProxyParams { url }): Query<ImageProxyParams>,
) -> impl IntoResponse {
    let response = reqwest::get(url).await.unwrap();
    let content_type = response
        .headers()
        .get("Content-Type")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let data = response.bytes().await.unwrap();

    let headers = [
        (header::CONTENT_TYPE, content_type),
        (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".to_owned()),
        (header::ACCESS_CONTROL_ALLOW_METHODS, "GET".to_owned()),
    ];

    (headers, data)
}

async fn fetch_image(image_url: &str) -> PhotonImage {
    let response = reqwest::get(image_url).await.unwrap();
    let body = response.bytes().await.unwrap();
    photon::native::open_image_from_bytes(&body).unwrap()
}
