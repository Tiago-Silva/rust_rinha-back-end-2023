use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post};
use crate::entity::product::NewProduct;
use crate::repository::product_repository::ProductRepository;
use tokio::runtime::Runtime;

pub struct ProductController {
    pub repo: ProductRepository,
}

impl ProductController {

    // pub async fn new() -> Self {
    //     let repo = ProductRepository::new().await.unwrap();
    //     Self { repo }
    // }

    pub fn routes() -> Router {

        Router::new()
            .route("/", get(Self::get_all_products))
            .route("/", post(Self::create_product))
    }

    async fn get_all_products(
    ) -> impl IntoResponse {
        println!("{:?}", "get_all_products");
        (StatusCode::OK, Json("Find all products"))
    }

    async fn create_product(
        Json(new_product): Json<NewProduct>
    ) -> impl IntoResponse {
        println!("{:?}", new_product.name);

        let repo = ProductRepository::new().await.unwrap();

        let product_id = repo.create_product(new_product).await.unwrap();
        let product_created = repo.get_product_by_id(product_id as i32).await.unwrap();

        (StatusCode::OK, Json(product_created))
    }
}