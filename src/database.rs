use crate::config::Configurations;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

#[derive(Debug)]
pub struct DatabaseState {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

pub fn get_connection_pool(config: &Configurations) -> DatabaseState {
    let url = get_database_url(config);
    let manager = ConnectionManager::<PgConnection>::new(url);

    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to build connection pool");

    DatabaseState { pool }
}

fn get_database_url(config: &Configurations) -> String{
    format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database.user,
        config.database.password,
        config.database.host,
        config.database.port,
        config.database.name
    )
}