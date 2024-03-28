mod controller;
use controller::person_controller::{Person, NewPerson, AppState};

use std::{
    collections::HashMap,
    sync::Arc
};
use axum::{routing::{get, post}, Router, extract::State, http::StatusCode, response::IntoResponse, Json};
use axum::extract::Path;
use serde::{Deserialize, Serialize};
use time::{macros::date, Date};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;


#[tokio::main]
async fn main() {
    let mut people: HashMap<Uuid, Person> = HashMap::new();

    let person = Person {
        id: Uuid::now_v7(),
        name: String::from("Roberto"),
        nick: String::from("Rob"),
        birth_date: date!(1986 - 03 - 31),
        stack: None
    };
    // vec!["Rust".to_string(), "Java".to_string() ]

    println!("{}", person.id);

    people.insert(person.id, person);

    let app_state: AppState = Arc::new(RwLock::new(people));

    let app = Router::new()
        .route("/pessoas", get(Person::search_people))
        .route("/pessoas/:id", get(Person::find_person))
        .route("/pessoas", post(Person::create_person))
        .route("/contagem-pessoas", get(Person::count_person))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
