use crate::{
    core::{
        model::{User, UserId},
        service::UserService,
    },
    dao::user::{ConnectionsPool, UserDao},
    http::dto::Response,
};
use actix_web::{get, post, web, Responder};

#[get("/api/v1/user/{name}")]
pub async fn user(pool: web::Data<ConnectionsPool>, name: web::Path<String>) -> impl Responder {
    todo!();
    ""
    // let user_dao = UserDao::new(pool.get_ref().clone());
    // let user_id = UserId(id.into_inner());
    //
    // let response = match web::block(move || user_dao.get_by_id(user_id)).await {
    //     Ok(user) => Response::from_user(user, "User exists"),
    //     Err(err) => Response::from_error(err),
    // };
    // web::Json(response)
}

#[get("/api/v1/group/{name}")]
pub async fn group(pool: web::Data<ConnectionsPool>, name: web::Path<String>) -> impl Responder {
    todo!();
    ""
}

#[get("/api/v1/space/{name}")]
pub async fn space(pool: web::Data<ConnectionsPool>, name: web::Path<String>) -> impl Responder {
    todo!();
    ""
}
