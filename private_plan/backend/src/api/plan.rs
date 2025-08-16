use actix_web::{HttpResponse, web};

use crate::db::DbPool;
use crate::models::plan::Plan;
use crate::models::response::ApiResponse;
use crate::utils::errors::AppError;

pub async fn create_plan(
    pool: web::Data<DbPool>,
    plan_data: web::Json<Plan>,
) -> Result<HttpResponse, AppError> {
    let _ = pool;
    let _ = plan_data;
    // Logic to create a new plan
    Ok(HttpResponse::Ok().json(ApiResponse::success(Some(()))))
}

pub async fn get_plans(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let _ = pool;
    // let _ = plan_data;
    // Logic to create a new plan
    Ok(HttpResponse::Ok().json(ApiResponse::success(Some(()))))
}

pub async fn get_plan_by_id(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let _ = pool;
    // let _ = plan_data;
    // Logic to create a new plan
    Ok(HttpResponse::Ok().json(ApiResponse::success(Some(()))))
}
pub async fn update_plan_by_id(
    pool: web::Data<DbPool>,
    plan_id: web::Path<i32>,
    plan_data: web::Json<Plan>,
) -> Result<HttpResponse, AppError> {
    let _ = pool;
    let _ = plan_id;
    let _ = plan_data;
    // Logic to update a plan by ID
    Ok(HttpResponse::Ok().json(ApiResponse::success(Some(()))))
}

pub async fn delete_plan_by_id(
    pool: web::Data<DbPool>,
    plan_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let _ = pool;
    let _ = plan_id;
    // Logic to delete a plan by ID
    Ok(HttpResponse::Ok().json(ApiResponse::success(Some(()))))
}
