use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;
use crate::error::TaskError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub created_at: SystemTime,
    pub due_at: Option<SystemTime>,
}


impl Task {
    pub fn new(title: String, description: Option<String>, priority: TaskPriority, due_at: Option<SystemTime>) -> Result<Self, TaskError> {
        let title = title.trim().to_string();

        if title.is_empty() {
            return Err(TaskError::EmptyTitle);
        }
        if title.len() > 100 {
            return Err(TaskError::TitleTooLong {max: 100});
        }

        Ok(Self {
            
            id: Uuid::new_v4(),
            title,
            description,
            status: TaskStatus::Todo,
            priority,
            created_at: SystemTime::now(),
            due_at, 
        })
    }

    pub fn update_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    pub fn update(&mut self, title: Option<String>,
        description: Option<Option<String>>) -> Result<(), TaskError> {

        if title.is_none() && description.is_none() {
            return Err(TaskError::NoFieldsToUpdate);
        }

        if let Some(title) = title {
            let title = title.trim().to_string();

            if title.is_empty() {
                return Err(TaskError::EmptyTitle);

            }


            if title.len() > 100 {
                return Err(TaskError::TitleTooLong {max: 100});

            }

            self.title = title;

        }

        Ok(());
    }

}
