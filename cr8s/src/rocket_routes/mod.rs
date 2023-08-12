use diesel::PgConnection;

pub mod crates;
pub mod rustaceans;

#[rocket_sync_db_pools::database("pg_cr8s")]
pub struct DbConnection(PgConnection);
