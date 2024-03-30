use std::collections::HashMap;
use std::sync::Arc;
use axum::{routing::{get, post}, Router, extract::{ State, Path }, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use time::{ Date };
use time::macros::date;
use tokio::sync::RwLock;
use uuid::Uuid;

time::serde::format_description!(date_format, Date, "[day]-[month]-[year]");

pub type AppState = Arc<RwLock<HashMap<Uuid, Person>>>;

#[derive(Clone, Serialize)]
pub struct Person {
    pub id: Uuid,
    #[serde(rename = "nome")]
    pub name: String,
    #[serde(rename = "apelido")]
    pub nick: String,
    #[serde(rename = "nascimento", with = "date_format")]
    pub birth_date: Date,
    pub stack: Option<Vec<String>>
}

#[derive(Clone, Deserialize)]
pub struct NewPerson {
    #[serde(rename = "nome")]
    pub name: String,
    #[serde(rename = "apelido")]
    pub nick: String,
    #[serde(rename = "nascimento", with = "date_format")]
    pub birth_date: Date,
    pub stack: Option<Vec<String>>
}

impl Person {

    pub fn routes() -> Router {
        let mut people: HashMap<Uuid, Person> = HashMap::new();

        let person = Person {
            id: Uuid::now_v7(),
            name: String::from("Roberto"),
            nick: String::from("Rob"),
            birth_date: date!(1986 - 03 - 31),
            stack: None
        };

        println!("{}", person.id);

        people.insert(person.id, person);

        let app_state: AppState = Arc::new(RwLock::new(people));

        Router::new()
            .route("/", get(Self::search_people))
            .route("/:id", get(Self::find_person))
            .route("/", post(Self::create_person))
            .route("/contagem-pessoas", get(Self::count_person))
            .with_state(app_state)
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

    async fn search_people(state: State<AppState>) -> impl IntoResponse {
        (StatusCode::OK, "Busca pessoas");
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
}