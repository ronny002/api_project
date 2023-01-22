// data access object

use async_trait::async_trait;
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
        AnswersDaoImpl { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&answer.question_uuid)
            .map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        let record = sqlx::query!(
            "INSERT INTO answers ( question_uuid, content )
            VALUES ( $1, $2 )
            RETURNING *",
            uuid,
            answer.content
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(e) = e {
                if e.code()
                    .unwrap()
                    .contains(postgres_error_codes::FOREIGN_KEY_VIOLATION)
                {
                    DBError::InvalidUUID(e.to_string())
                } else {
                    DBError::Other(Box::new(e))
                }
            } else {
                DBError::Other(Box::new(e))
            }
        })?;

        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&answer_uuid)
            .map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        sqlx::query!("DELETE FROM answers WHERE answer_uuid = $1", uuid)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid)
            .map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        let records = sqlx::query!("SELECT * FROM answers WHERE question_uuid = $1", uuid)
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        let answers = records
            .iter()
            .map(|x| AnswerDetail {
                answer_uuid: x.answer_uuid.to_string(),
                question_uuid: x.question_uuid.to_string(),
                content: x.content.clone(),
                created_at: x.created_at.to_string(),
            })
            .collect();

        Ok(answers)
    }
}
