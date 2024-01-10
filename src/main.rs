extern crate pretty_env_logger;

#[macro_use]
extern crate rocket;
extern crate log;

mod cors;
mod handlers;
mod models;
mod persistance;

use std::env;
use cors::*;
use handlers::*;
use dotenvy::dotenv;
use persistance::{questions_dao::{QuestionsDaoImpl, QuestionsDao}, answers_dao::{AnswersDaoImpl, AnswersDao}};
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

    let questions_dao = QuestionsDaoImpl::new(pg_pool.clone());
    let answers_dao = AnswersDaoImpl::new(pg_pool.clone());

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
        .manage(Box::new(questions_dao) as Box<dyn QuestionsDao + Send + Sync>)
        .manage(Box::new(answers_dao) as Box<dyn AnswersDao + Send + Sync>)
}