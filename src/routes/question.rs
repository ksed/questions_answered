use crate::store::Store;
use crate::types::pagination::{extract_pagination, Pagination};
use crate::types::question::{Question, QuestionId};
use handle_errors::Error;
use std::collections::HashMap;
use tracing::{info, instrument};
use warp::http::StatusCode;
use warp::{Rejection, Reply};

fn check_bounds(mut bounds: Pagination, curr_len: i32) -> Result<Pagination, Error> {
    if bounds.start < 1 || bounds.end < 1 || bounds.start as i32 > curr_len {
        return Err(Error::ParametersOutOfRange);
    } else if bounds.start > bounds.end {
        return Err(Error::SequencingError);
    } else if bounds.end as i32 > curr_len {
        bounds.end = curr_len as usize;
    } else if bounds.start != bounds.end {
        bounds.end -= 1;
    }
    bounds.start -= 1;
    Ok(bounds)
}

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    info!("querying questions");
    let res: Vec<Question> = store.questions.read().await.values().cloned().collect();

    if !params.is_empty() {
        let pagination = check_bounds(extract_pagination(params)?, res.len() as i32)?;
        info!(pagination = true);
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        info!(pagination = false);
        Ok(warp::reply::json(&res))
    }
}

pub async fn add_question(store: Store, question: Question) -> Result<impl Reply, Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);

    Ok(warp::reply::with_status("Question added.", StatusCode::OK))
}

pub async fn update_question(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status(
        "Question updated.",
        StatusCode::OK,
    ))
}

pub async fn delete_question(id: String, store: Store) -> Result<impl Reply, Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status(
            "Question deleted.",
            StatusCode::OK,
        )),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}
