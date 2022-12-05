use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use chrono::Utc;
use uuid::Uuid;

//Model for user
#[derive(FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub password: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub status: bool,
}

//Model for create user
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub password: String,
    pub status: bool,
}

//Model for update user
#[derive(Deserialize)]
pub struct UpdateUser {
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub status: bool,
}
