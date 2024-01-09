extern crate pretty_env_logger;

#[macro_use]
extern crate rocket;
extern crate log;

mod cors;
mod handlers;
mod models;
mod utils;
// mod db;

use std::env;
use cors::*;
use handlers::*;
// use log::{info, warn, error, debug};
use log::info;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().expect("Failed to load .env");

    let pg_pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("Env var is not set"))
        .await
        .expect("Failed to connect to db");

    let question_res = sqlx::query!("select * from questions")
        .fetch_all(&pg_pool)
        .await
        .unwrap();

    info!("*********************Questions in db*********************");
    info!("{:?}", question_res);

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
}