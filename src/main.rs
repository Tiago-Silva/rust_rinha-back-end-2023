mod controller;
use controller::person_controller::{Person, NewPerson};

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

type AppState = Arc<RwLock<HashMap<Uuid, Person>>>;

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
        .route("/pessoas", get(search_people))
        .route("/pessoas/:id", get(find_person))
        .route("/pessoas", post(create_person))
        .route("/contagem-pessoas", get(count_person))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn search_people(state: State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, "Busca pessoas");
}

async fn find_person(
    State(people): State<AppState>,
    Path(person_id): Path<Uuid>
) -> impl IntoResponse {
    match people.read().await.get(&person_id) {
        Some(person) => Ok(Json(person.clone())),
        None => Err(StatusCode::NOT_FOUND)
    }
}

async fn create_person(
    State(people): State<AppState>,
    Json(new_person): Json<NewPerson>
) -> impl IntoResponse {
    let id = Uuid::now_v7();
    let person = Person {
        id,
        name: new_person.name,
        nick: new_person.nick,
        birth_date: new_person.birth_date,
        stack: new_person.stack
    };

    people.write().await.insert(id, person.clone());

    (StatusCode::OK, Json(person))
}

async fn count_person(state: State<AppState>) -> impl IntoResponse {
    let count = state.read().await.len();
    (StatusCode::OK, Json(count));
}
