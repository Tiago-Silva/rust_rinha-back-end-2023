use serde::{Deserialize, Serialize};
use serde::de::{Deserializer, Visitor};
use std::str::FromStr;

#[derive(sqlx::FromRow, PartialEq, Debug, Serialize, Ord, Eq, PartialOrd)]
pub struct Product {
    pub idproduct: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<String>,
    pub category: Option<String>,
    pub url_image: Option<String>,
}

#[derive(Deserialize)]
pub struct NewProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<String>,
    pub category: Option<String>,
    pub url_image: Option<String>,
}