#[macro_use]
extern crate diesel;

use actix_web::{error, get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::{
    prelude::*,
    r2d2::{self, Pool},
};
use uuid::Uuid;

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let user_uid = user_uid.into_inner();

    let user = web::block(move || {
        let mut conn = pool.get()?;

        actions::find_user_by_uid(&mut conn, user_uid)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match user {
        Some(user) => HttpResponse::Ok().json(user),

        None => HttpResponse::NotFound().body(format!("No user found with UID: {user_uid}")),
    })
}

fn main() {
    println!("Hello, world!");
}
