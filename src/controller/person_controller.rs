use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use time::{ Date };
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
    pub async fn find_person(
        State(people): State<AppState>,
        Path(person_id): Path<Uuid>
    ) -> impl IntoResponse {
        match people.read().await.get(&person_id) {
            Some(person) => Ok(Json(person.clone())),
            None => Err(StatusCode::NOT_FOUND)
        }
    }

    pub async fn search_people(state: State<AppState>) -> impl IntoResponse {
        (StatusCode::OK, "Busca pessoas");
    }

    pub async fn create_person(
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

    pub async fn count_person(state: State<AppState>) -> impl IntoResponse {
        let count = state.read().await.len();
        (StatusCode::OK, Json(count));
    }
}