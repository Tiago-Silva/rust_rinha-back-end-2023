use sqlx::{Pool, MySql};
use crate::entity::product::{NewProduct, Product};

pub struct ProductRepository{
    connection: Pool<MySql>,
}

impl ProductRepository {
    pub fn new(connection: Pool<MySql>) -> Self {
        Self { connection }
    }

    pub async fn create_product(&self, new_product: NewProduct) -> Result<u64, sqlx::Error> {
        let product_id = sqlx::query!(
            r#"
            INSERT INTO product (name, description, price, category, url_image)
            VALUES (?, ?, ?, ?, ?)
            "#,
            new_product.name,
            new_product.description,
            new_product.price,
            new_product.category,
            new_product.url_image
        )
            .execute(&self.connection)
            .await?
            .last_insert_id();
        Ok(product_id)
    }

    pub async fn get_products(&self) -> Result<Vec<Product>, sqlx::Error> {
        // let products = sqlx::query_as::<_, Product>("SELECT * FROM product")
        //     .fetch_all(&self.connection)
        //     .await?;

        Ok(vec![])
    }

    pub async fn get_product_by_id(&self, id: i32) -> Result<Product, sqlx::Error> {
        let product = sqlx::query_as!(
        Product,
        r#"
        SELECT * FROM product WHERE idproduct = ?
        "#,
        id
    )
            .fetch_one(&self.connection)
            .await?;
        Ok(product)
    }
}