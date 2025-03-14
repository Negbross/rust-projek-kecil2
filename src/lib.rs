// pub mod model;
// pub mod schema;
//
// use std::env;
// use diesel::{Connection, Insertable, IntoSql, PgConnection, RunQueryDsl, SelectableHelper};
// use dotenvy::dotenv;
//
// use model::{NewProduct, Products};
// use schema::products;
//
// pub fn establish_connection() -> Result<PgConnection, diesel::result::ConnectionError> {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url)
// }
//
// pub fn insert_product(
//     conn: &mut PgConnection,
//     product_name: &str,
//     description: &str,
//     quantity: i32,
// ) -> Result<Products, diesel::result::Error> {
//     let new_product = NewProduct { product_name, description, quantity };
//     diesel::insert_into(products::table)
//         .values(&new_product)
//         .returning(Products::as_returning())
//         .get_result(conn)
// }
//
// pub fn get_products(conn: &mut PgConnection) -> Result<Vec<Products>, diesel::result::Error> {
//     use schema::products::dsl::*;
//     products.load::<Products>(conn)
// }