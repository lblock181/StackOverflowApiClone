// extern crate sqlx;

// use sqlx::{Pool, Database};
// // use sqlx::pool::PoolOptions;

// use sqlx::postgres::PgPoolOptions;

// pub async fn init_pool_connection(db_url: &str, max_conn: u32) -> Result<Pool<Database>, sqlx::Error> {
//     PgPoolOptions::new()
//         .max_connections(max_conn)
//         .connect(db_url)
//         .await
// }