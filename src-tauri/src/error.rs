use serde::Serialize;
use thiserror::Error;

use crate::domain::task::TaskStatus;

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

    #[error("Invalid status transition from {from} to {to}")]
    InvalidStatusTransition { from: TaskStatus, to: TaskStatus },
}
