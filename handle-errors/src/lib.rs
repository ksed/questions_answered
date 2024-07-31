use warp::reject::Reject;
use warp::{Rejection, Reply};
use warp::body::BodyDeserializeError;
use warp::cors::CorsForbidden;
use warp::http::StatusCode;

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    ParametersOutOfRange,
    SequencingError,
    QuestionNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Can't parse parameter: {}.", err),
            Error::MissingParameters => write!(f, "Missing parameter."),
            Error::ParametersOutOfRange => write!(f, "Your start and/or end value was not found in the store records."),
            Error::SequencingError => write!(f, "You starting value must be less than the end value."),
            Error::QuestionNotFound => write!(f, "Question was not found."),
        }
    }
}

impl Reject for Error {}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("Bad Request: {:?}.", r);
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>(){
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found!".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
