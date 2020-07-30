use thiserror::Error;

use lambda_runtime::error::HandlerError;
use rusoto_core::RusotoError;
use rusoto_dynamodb::{GetItemError, PutItemError};

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("{0}")]
    LogicError(String),
    #[error("`{0}` is not found")]
    NotFound(String),
    #[error("permission denied.")]
    PermissionDenied,
    #[error("`{0}` is not implemented")]
    NoImplement(String),

    #[error("dynamodb get item error")]
    DynamoDbGetItem(#[from] RusotoError<GetItemError>),
    #[error("dynamodb put item error")]
    DynamoDbPutItem(#[from] RusotoError<PutItemError>),

    #[error("unknown error")]
    Unknown,
}

impl From<ApplicationError> for HandlerError {
    fn from(err: ApplicationError) -> HandlerError {
        return HandlerError::from(err.to_string().as_str());
    }
}
