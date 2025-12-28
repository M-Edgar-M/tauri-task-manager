use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum TaskError {
    #[error("Title cannot be empty")]
    EmptyTitle,

    #[error("Title is too long (max {max} characters)")]
    TitleTooLong { max: usize },

    #[error("Task not found")]
    NotFound,

    #[error("No fields to update")]
    NoFieldsToUpdate,

    #[error("An error occurred during persistence")]
    Persistence,

    #[error("Invalid UUID")]
    InvalidUuid,
}
