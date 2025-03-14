use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateProductRequest{
    pub product_name: String,
    pub description: String,
    pub quantity: i32
}