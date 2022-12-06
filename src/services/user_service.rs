use actix_web::web;

use crate::models::user_model::{User, UpdateUser, CreateUser};
use crate::utils::auth_utils::hash_password;
use crate::AppState;

use uuid::Uuid;
use chrono::Utc;

pub async fn get_all_user_information_service(state: web::Data<AppState>) -> Result<Vec<User>, sqlx::Error>{
    let users = sqlx::query_as::<_, User>("SELECT * FROM end_user")
        .fetch_all(&state.db)
        .await
        .map_err(|err| {
            println!("Error getting all users: {}", err);
            err
        })?;
    Ok(users)
}

pub async fn get_user_information_by_id_service(
    state: web::Data<AppState>,
    id: String,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM end_user WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(|err| {
            println!("Error getting user by id: {}", err);
            err
        })?;
    Ok(user)
}

pub async fn create_user_service(state: web::Data<AppState>, user: web::Json<CreateUser>) -> Result<User, sqlx::Error> {
    let id = Uuid::new_v4();
    let created_and_updated_at = Utc::now();
    let hashed_password: String = hash_password(&user.password.as_str()).unwrap();

    let user = sqlx::query_as::<_,User>("INSERT INTO end_user (id, username, email, first_name, middle_name, last_name, password, created_at, updated_at, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *")
        .bind(id)
        .bind(user.username.clone())
        .bind(user.email.clone())
        .bind(user.first_name.clone())
        .bind(user.middle_name.clone())
        .bind(user.last_name.clone())
        .bind(hashed_password.clone())
        .bind(created_and_updated_at)
        .bind(created_and_updated_at)
        .bind(user.status.clone())
        .fetch_one(&state.db)
        .await
        .map_err(|err| {
            println!("Error creating user: {}", err);
            err
        })?;
    Ok(user)
}

pub async fn update_user_information_service(
    state: web::Data<AppState>,
    id: String,
    user: web::Json<UpdateUser>,
) -> Result<User, sqlx::Error> {
    let updated_at = Utc::now();

    let user = sqlx::query_as::<_, User>("UPDATE end_user SET username = $1, email = $2, first_name = $3, middle_name = $4, last_name = $5, updated_at = $6, status = $7 WHERE id = $8 RETURNING *")
        .bind(user.username.clone())
        .bind(user.email.clone())
        .bind(user.first_name.clone())
        .bind(user.middle_name.clone())
        .bind(user.last_name.clone())
        .bind(updated_at)
        .bind(user.status)
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(|err| {
            println!("Error updating user: {}", err);
            err
        })?;
    Ok(user)
}

pub async fn delete_user_service(
    state: web::Data<AppState>,
    id: String,
) -> Result<(), sqlx::Error> {
    match sqlx::query("DELETE FROM end_user WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(sqlx::Error::RowNotFound),
    }
}