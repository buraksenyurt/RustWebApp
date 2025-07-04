use actix_web::web;
use actix_web::web::{delete, get, post, put};

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/", get().to(get::fetch_all))
            .route(
                "filter/by/status/{status}",
                get().to(get::fetch_all_by_status),
            )
            .route(
                "/filter/by/size/{size}",
                get().to(get::fetch_all_by_size_grater_than),
            )
            .route("/{title}", get().to(get::fetch_by_id))
            .route("/", post().to(create::create_work_item))
            .route("/", put().to(update::update_work_item))
            .route("/{title}", delete().to(delete::delete_by_title)),
    );
}
