use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use tracing::{info, instrument};
use crate::database::DatabaseState;
use crate::model::{DeleteProduct, NewProduct, Products, UpdateProduct};
use diesel::prelude::*;
use crate::schema::products;

#[instrument]
pub async fn create_product(
    State(db): State<Arc<DatabaseState>>,
    Json(payload): Json<NewProduct>
) -> Result<Json<Products>, (StatusCode, String)> {
    let mut conn = db.pool.get().map_err(internal_error)?;

    info!("Create product: {}", &payload.product_name);
    let new_product = diesel::insert_into(products::table)
        .values(&payload)
        .returning(Products::as_returning())
        .get_result(&mut conn)
        .map_err(internal_error)?;

    Ok(Json(new_product))
}

pub async fn delete_product(
    State(db): State<Arc<DatabaseState>>,
    Json(payload): Json<DeleteProduct>
) -> Result<Json<String>, (StatusCode, String)> {
    let mut conn = db.pool.get().map_err(internal_error)?;

    info!("Delete product: {:?}", &payload);
    diesel::delete(products::table.find(payload.id))
        .returning(Products::as_returning())
        .get_result(&mut conn)
        .map_err(internal_error)?;

    Ok(Json("sucess delete".to_string()))
}

#[axum::debug_handler]
pub async fn update_product(
    State(db): State<Arc<DatabaseState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateProduct>
) -> Result<Json<Products>, (StatusCode, String)> {
    let mut conn = db.pool.get().map_err(internal_error)?;

    info!("Update product: {}", &payload.product_name);
    let update_product = diesel::update(products::table.find(id))
        .set(&payload)
        .returning(Products::as_returning())
        .get_result(&mut conn)
        .map_err(internal_error)?;

    Ok(Json(update_product))
}

pub async fn get_product(
        State(db): State<Arc<DatabaseState>>,
        Path(id) : Path<i32>
    )
    -> Result<Json<Products>, (StatusCode, String)> {
    let mut conn = db.pool.get().map_err(internal_error)?;

    info!("Get product: {}", &id);
    let get_product = products::dsl::products
        .find(id)
        .select(Products::as_select())
        .first(&mut conn)
        .optional();

    match get_product {
        Ok(Some(product)) => Ok(Json(product)),
        Ok(None) => Err(not_found_error("Not found")),
        Err(err) => Err(internal_error(err)),
    }
}

pub async fn list_products(
    State(db): State<Arc<DatabaseState>>
) -> Result<Json<Vec<Products>>, (StatusCode, String)> {
    let mut conn = db.pool.get().map_err(internal_error)?;

    info!("Retrieving products");
    let result = products::dsl::products
        .select(Products::as_select())
        .load(&mut conn)
        .optional();

    match result {
        Ok(Some(products)) => Ok(Json(products)),
        Ok(None) => Err(not_found_error("No record yet")),
        Err(err) => Err(internal_error(err)),
    }
}

/* Map any error into a `500 Internal Server Error` */
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

// Map str into a `404 Internal Server Error`
fn not_found_error(msg: &str) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, msg.to_string())
}