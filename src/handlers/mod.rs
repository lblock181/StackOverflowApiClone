use rocket::{serde::json::Json};

use crate::models::*;

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
) -> Json<QuestionDetail> {
    // request has title and descr
    // return question_uuid, title, description, created_at (serialize question detail)
    todo!()
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    // req -> no body
    // serialize all questionDetails into vec
    todo!()
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(question_uuid: Json<QuestionId>) {
    // req -> question_uuid
    // response -> no body but 200 resp code
    todo!()
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
) -> Json<AnswerDetail> {
    todo!()
}

#[get("/answers")]
pub async fn read_answers() -> Json<Vec<AnswerDetail>> {
    todo!()
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(answer_uuid: Json<AnswerId>) {
    todo!()
}
// TODO: Create a POST route to /answer which accepts an `Answer` and returns `AnswerDetail` as JSON.
//       The handler function should be called `create_answer`.
//       
//       hint: this function should look very similar to the create_question function above

// TODO: Create a GET route to /answers which accepts an `QuestionId` and returns a vector of `AnswerDetail` as JSON.
//       The handler function should be called `read_answers`.
//       
//       hint: this function should look very similar to the read_questions function above

// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//       
//       hint: this function should look very similar to the delete_question function above