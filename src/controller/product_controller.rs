use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post};
use crate::entity::product::NewProduct;
use crate::repository::product_repository::ProductRepository;
use crate::repository::repository::Repository;

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

    // Em product_controller.rs
    async fn create_product(
        Json(new_product): Json<NewProduct>
    ) -> impl IntoResponse {
        println!("{:?}", new_product.name);

        let repo = Repository::new().await.unwrap();
        let product_repository = ProductRepository::new(repo.get_connection());

        let product_id = product_repository.create_product(new_product).await.unwrap();
        let product_created = product_repository.get_product_by_id(product_id as i32).await.unwrap();

        (StatusCode::OK, Json(product_created))
    }
}