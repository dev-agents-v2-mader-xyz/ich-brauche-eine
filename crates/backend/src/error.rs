use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("not found")]
    NotFound,
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let status = match &self {
            ApiError::NotFound => Status::NotFound,
            ApiError::Conflict(_) => Status::Conflict,
            ApiError::BadRequest(_) => Status::BadRequest,
            ApiError::Unauthorized => Status::Unauthorized,
            ApiError::Forbidden => Status::Forbidden,
            ApiError::Database(_) => Status::InternalServerError,
        };
        let body = match &self {
            ApiError::Database(_) => "internal server error".to_string(),
            other => other.to_string(),
        };
        tracing::error!("api error: {}", self);
        Response::build_from(body.respond_to(req)?)
            .status(status)
            .ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn database_error_is_opaque() {
        let e = ApiError::Database(sqlx::Error::RowNotFound);
        assert_eq!(e.to_string(), "database error: no rows returned by a query that expected to return at least one row");
    }
}
