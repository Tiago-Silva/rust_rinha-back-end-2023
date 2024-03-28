mod controller;
use controller::person_controller::{Person};

use axum::{Router, response::IntoResponse};
use serde::{Deserialize, Serialize};


#[tokio::main]
async fn main() {

    let app = Router::new()
        .nest("/pessoas", Person::routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
