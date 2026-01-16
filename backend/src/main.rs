use std::{any::Any, panic::UnwindSafe};

use axum::{
    Router,
    body::Body,
    extract::Query,
    http::header,
    response::{IntoResponse, Response},
    routing::get,
};
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
        transformers::exercise_2::transform(photon_image, do_one_thing, value);
    })
    .await
}

async fn process_image<F>(image_url: &str, f: F) -> impl IntoResponse + use<F>
where
    F: FnOnce(&mut PhotonImage) + UnwindSafe,
{
    let response = reqwest::get(image_url).await.unwrap();
    let body = response.bytes().await.unwrap();

    let res = std::panic::catch_unwind(|| {
        let mut photon_image = photon::native::open_image_from_bytes(&body).unwrap();

        f(&mut photon_image);

        photon_image.get_bytes_jpeg(80)
    });

    match res {
        Ok(output) => Response::builder()
            .status(200)
            .header(header::CONTENT_TYPE, "image/jpeg")
            .body(Body::from(output))
            .unwrap(),
        Err(err) => Response::builder()
            .status(500)
            .header(header::CONTENT_TYPE, "text/plain")
            .body(Body::from(panic_to_string(err)))
            .unwrap(),
    }
}

fn panic_to_string(payload: Box<dyn Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.to_string()
    } else {
        "unknown error".to_string()
    }
}
