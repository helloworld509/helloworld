use crate::api::auth::{login, me, register};
use crate::api::plan::{
    create_plan, delete_plan_by_id, get_plan_by_id, get_plans, update_plan_by_id,
};
use actix_web::web;
/*
/api/auth/register post
/api/auth/login post
/api/auth/me get

plan

/api/plans post create plan
/api/plans get list plans
/api/plans/{id} get get plan by id
/api/plans/{id} put update plan by id
/api/plans/{id} delete delete plan by id

*/
async fn _default_handler() -> &'static str {
    "Welcome to the API!"
}
pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(register))
                    .route("/me", web::get().to(me))
                    .route("/login", web::post().to(login)),
            )
            .service(
                web::scope("/plans")
                    .route("", web::post().to(create_plan))
                    .route("/{id}", web::delete().to(delete_plan_by_id))
                    .route("/{id}", web::get().to(get_plan_by_id))
                    .route("", web::get().to(get_plans))
                    .route("/{id}", web::put().to(update_plan_by_id)),
            ),
    );
}
