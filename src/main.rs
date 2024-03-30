mod controller;
mod entity;
mod repository;

use controller::person_controller::{Person};

use axum::{Router, response::IntoResponse};
use serde::{Deserialize, Serialize};
use controller::product_controller::ProductController;


#[tokio::main]
async fn main() {

    let app = Router::new()
        .nest("/pessoas", Person::routes())
        .nest("/product", ProductController::routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
