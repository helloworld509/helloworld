use crate::db::DbPool;
use crate::models::response::ApiResponse;
use crate::models::user::User;
use crate::utils::errors::AppError;
use actix_web::{HttpResponse, Result, web};

pub async fn register(
    pool: web::Data<DbPool>,
    _: web::Json<User>,
) -> Result<HttpResponse, AppError> {
    let _ = pool;
    Ok(HttpResponse::Ok().json(ApiResponse::success(Some(()))))
}

pub async fn login(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    // Login logic here
    let _ = pool;
    Ok(HttpResponse::Ok().json(ApiResponse::success(Some(()))))
}
pub async fn me(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    // Logic to get current user
    let _ = pool;
    Ok(HttpResponse::Ok().json(ApiResponse::success(Some(()))))
}
