use actix_web::{web, HttpResponse, Responder};

use crate::models::user_model::{User, UpdateUser, CreateUser};
use crate::utils::auth_utils::hash_password;
use crate::AppState;

use uuid::Uuid;
use chrono::Utc;

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/").route(web::get().to(|| async { HttpResponse::Ok().body("test") })),
    )
    .service(
        web::resource("/users")
        .route(web::get().to(get_all_user_information))
        .route(web::post().to(create_user))
    )
    .service(
        web::resource("/users/{id}")
            .route(web::get().to(get_user_information_by_id))
            .route(web::put().to(update_user_information_by_id))
            .route(web::delete().to(delete_user_information_by_id)),
    );
}

async fn get_all_user_information(state: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT * FROM end_user")
        .fetch_all(&state.db)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("No users found"),
    }
}

async fn get_user_information_by_id(
    state: web::Data<AppState>,
    id: web::Path<String>,
) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT * FROM end_user WHERE id = $1")
        .bind(id.into_inner())
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().json("No user found"),
    }
}

async fn create_user(state: web::Data<AppState>, user: web::Json<CreateUser>) -> impl Responder {
    let id = Uuid::new_v4();
    let created_and_updated_at = Utc::now();
    let hashed_password: String = hash_password(&user.password.as_str()).unwrap();

    match sqlx::query_as::<_,User>("INSERT INTO end_user (id, username, email, first_name, middle_name, last_name, password, created_at, updated_at, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *")
        .bind(id)
        .bind(user.username.clone())
        .bind(user.email.clone())
        .bind(user.first_name.clone())
        .bind(user.middle_name.clone())
        .bind(user.last_name.clone())
        .bind(hashed_password)
        .bind(created_and_updated_at)
        .bind(created_and_updated_at)
        .bind(user.status.clone())
        .fetch_one(&state.db)
        .await
    {
        Ok(user_created) => HttpResponse::Ok().json(user_created),
        Err(_) => HttpResponse::NotFound().json("Error creating user"),
    }
}

async fn update_user_information_by_id(state: web::Data<AppState>, id: web::Path<String>, user: web::Json<UpdateUser>) -> impl Responder {
    let update_time = Utc::now();
    match sqlx::query("UPDATE end_user SET username = $1, email = $2, first_name = $3, middle_name = $4, last_name = $5, updated_at = $6, status = $7 WHERE id = $8")
        .bind(user.username.clone())
        .bind(user.email.clone())
        .bind(user.first_name.clone())
        .bind(user.middle_name.clone())
        .bind(user.last_name.clone())
        .bind(update_time)
        .bind(user.status.clone())
        .bind(id.into_inner())
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("User updated"),
        Err(_) => HttpResponse::NotFound().json("No user found"),
    }
}

async fn delete_user_information_by_id(state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    match sqlx::query("DELETE FROM end_user WHERE id = $1")
        .bind(id.into_inner())
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("User deleted"),
        Err(_) => HttpResponse::NotFound().json("No user found"),
    }
}