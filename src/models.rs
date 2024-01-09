use std::fmt::Display;
use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionDetail {
    pub question_uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionId {
    pub question_uuid: String,
}

impl Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.question_uuid)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub question_uuid: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerDetail {
    pub answer_uuid: String,
    pub question_uuid: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct AnswerId {
    pub answer_uuid: String,
}

impl Display for AnswerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.answer_uuid)
    }
}

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Invalid UUID: {0}")]
    InvalidUUID(String),
    #[error("Database error")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub mod postgres_error_codes {
    pub const  FOREIGN_KEY_VIOLATION: &str = "23503";
}