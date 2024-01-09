use async_trait::async_trait;
use log::Record;
use sqlx::PgPool;

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self {
            db
        }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&answer.question_uuid)
            .map_err(|_| return DBError::InvalidUUID(answer.question_uuid))?;

        let record = sqlx::query!(
            "insert into answers (question_uuid, content)
            values ($1, $2)
            returning *",
            &uuid,
            &answer.content
        )
        .fetch_one(&self.db)
        .await
        .map_err(|_| return DBError::InvalidUUID(uuid.to_string()))?;

        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content.clone(),
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&answer_uuid)
            .map_err(|_| DBError::InvalidUUID(answer_uuid))?;

        sqlx::query!(
            "delete from answers where answer_uuid = $1", uuid
        )
        .execute(&self.db)
        .await
        .map_err(|e| return DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid)
        .map_err(|_| return DBError::InvalidUUID(question_uuid))?;

        let records = sqlx::query!(
            "select * from answers where question_uuid = $1", uuid
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| return DBError::Other(Box::new(e)))?;

        let answers = records
            .iter()
            .map(|r| AnswerDetail {
                answer_uuid: r.answer_uuid.to_string(),
                question_uuid: r.question_uuid.to_string(),
                content: r.content.clone(),
                created_at: r.created_at.to_string()
            })
            .collect();

        Ok(answers)
    }
}