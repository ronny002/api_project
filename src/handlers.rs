use rocket::{serde::json::Json, State};

use crate::{
    models::*,
    persistance::{answers_dao::AnswersDao, questions_dao::QuestionsDao},
};
mod handlers_inner;
use handlers_inner::*;

#[derive(Responder)]
pub enum APIError {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 500)]
    InternalError(String),
}

impl From<HandlerError> for APIError {
    fn from(value: HandlerError) -> Self {
        match value {
            HandlerError::BadRequest(s) => Self::BadRequest(s),
            HandlerError::InternalError(s) => Self::InternalError(s),
        }
    }
}

// ---- CRUD for Questions ---- Create, Read, Update, Delete

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<Json<QuestionDetail>, APIError> {
    let result = handlers_inner::create_question(question.into_inner(), questions_dao).await;

    match result {
        Ok(x) => Ok(Json(x)),
        Err(e) => Err(APIError::from(e)),
    }
}

#[get("/questions")]
pub async fn read_questions(
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<Json<Vec<QuestionDetail>>, APIError> {
    let result = handlers_inner::read_questions(questions_dao).await;

    match result {
        Ok(x) => Ok(Json(x)),
        Err(e) => Err(APIError::from(e)),
    }
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<(), APIError> {
    let result = handlers_inner::delete_question(question_uuid.into_inner(), questions_dao).await;

    if let Err(e) = result {
        return Err(APIError::from(e));
    }

    Ok(())
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>,
) -> Result<Json<AnswerDetail>, APIError> {
    let result = handlers_inner::create_answer(answer.into_inner(), answers_dao).await;

    match result {
        Ok(x) => Ok(Json(x)),
        Err(e) => Err(APIError::from(e)),
    }
}

#[get("/answers", data = "<question_uuid>")]
pub async fn read_answers(
    question_uuid: Json<QuestionId>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>,
) -> Result<Json<Vec<AnswerDetail>>, APIError> {
    let result = handlers_inner::read_answers(question_uuid.into_inner(), answers_dao).await;

    match result {
        Ok(x) => Ok(Json(x)),
        Err(e) => Err(APIError::from(e)),
    }
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>,
) -> Result<(), APIError> {
    let result = handlers_inner::delete_answer(answer_uuid.into_inner(), answers_dao).await;

    if let Err(e) = result {
        return Err(APIError::from(e));
    }

    Ok(())
}
