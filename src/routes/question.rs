use crate::store::Store;
use crate::types::pagination::{extract_pagination, Pagination};
use crate::types::question::{NewQuestion, Question};
use handle_errors::Error;
use std::collections::HashMap;
use tracing::{event, info, instrument, Level};
use warp::http::StatusCode;

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "questions_answered", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }

    info!(pagination = false);
    let res: Vec<Question> = match store
        .get_questions(pagination.limit, pagination.offset)
        .await {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(
                Error::DatabaseQueryError(e)
            )),
        };

    Ok(warp::reply::json(&res))
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.add_question(new_question).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
    }

    Ok(warp::reply::with_status("Question added.", StatusCode::OK))
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.update_question(question, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    }
}

pub async fn delete_question(
    id: i32,
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.delete_question(id).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
    }

    Ok(warp::reply::with_status(
        format!("Question {} deleted.", id),
        StatusCode::OK,
    ))
}
