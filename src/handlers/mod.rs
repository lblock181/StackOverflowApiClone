use rocket::serde::json::Json;

use crate::utils::{generate_datetime_string, generate_uuid_string};
use crate::models::*;

mod handlers_inner;

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(question: Json<Question>,) -> Json<QuestionDetail> {
    // request has title and descr
    // return question_uuid, title, description, created_at (serialize question detail)
    let question = Json::into_inner(question);
    let q_uuid: String = generate_uuid_string();
    Json(QuestionDetail{
        question_uuid: q_uuid,
        title: question.title,
        description: question.description,
        created_at: generate_datetime_string(),
    })
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    // req -> no body
    // serialize all questionDetails into vec
    Json(
        vec![
            QuestionDetail {
                question_uuid: generate_uuid_string(),
                title: "dummytile".to_owned(),
                description: "descriptions".to_owned(),
                created_at: generate_datetime_string(),
            },
            QuestionDetail {
                question_uuid: generate_uuid_string(),
                title: "dummytile2".to_owned(),
                description: "descriptions2".to_owned(),
                created_at: generate_datetime_string(),
            }
        ]
    )
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(question_uuid: Json<QuestionId>) {
    // req -> question_uuid
    // response -> no body but 200 resp code
    let question_uuid = Json::into_inner(question_uuid);
    println!("Deleting question {}", question_uuid);
    // empty fn for now until db connection setup
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
) -> Json<AnswerDetail> {
    let answer = Json::into_inner(answer);
    Json(
        AnswerDetail{
            question_uuid: answer.question_uuid,
            answer_uuid: generate_uuid_string(),
            content: answer.content,
            created_at: generate_datetime_string(),
        }
    )
}

#[get("/answers")]
pub async fn read_answers() -> Json<Vec<AnswerDetail>> {
    Json(
        vec![
            AnswerDetail {
                answer_uuid: "a".to_owned(),
                question_uuid: "12".to_owned(),
                content: "this is content for a".to_owned(),
                created_at: generate_datetime_string(),
            },
            AnswerDetail {
                answer_uuid: "b".to_owned(),
                question_uuid: "1".to_owned(),
                content: "this is content for b".to_owned(),
                created_at: generate_datetime_string(),
            }
        ]
    )
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(answer_uuid: Json<AnswerId>) {
    let answer_id = Json::into_inner(answer_uuid);
    println!("Deleting answer {}", answer_id);
    // left empty as no db to clear. Emtpy fn will return 200 response
}