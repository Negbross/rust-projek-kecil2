use diesel::prelude::*;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    DbError(#[from] diesel::result::Error),
}

#[derive(Serialize)]
pub struct ErrorResponse{
    pub error: String,
}

#[derive(Queryable, Serialize, Selectable, Debug)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Products {
    pub id: i32,
    pub product_name: String,
    pub description: String,
    pub quantity: i32
}

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = crate::schema::products)]
pub struct NewProduct {
    pub product_name: String,
    pub description: String,
    pub quantity: i32,
}

#[derive(Insertable,Queryable,Selectable,  Debug, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::products)]
pub struct DeleteProduct {
    pub id: i32
}

#[derive(AsChangeset, Debug, Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::products)]
pub struct UpdateProduct {
    pub product_name: String,
    pub description: Option<String>,
    pub quantity: Option<i32>,
}