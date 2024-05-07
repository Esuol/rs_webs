use actix_files::NamedFile;
use actix_session::Session;
use actix_web::{
    dev, error, http::StatusCode, middleware::ErrorHandlerResponse, web, Error, HttpResponse,
    Responder, Result,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use tera::{Context, Tera};

use crate::{
    db,
    session::{self, FlashMessage},
};

pub async fn index(
    pool: web::Data<SqlitePool>,
    tmpl: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let tasks = db::get_all_tasks(&pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    let mut context = Context::new();
    context.insert("tasks", &tasks);

    if let Some(flash) = session::get_flash(&session)? {
        context.insert("msg", &(flash.kind, flash.message));
        session::clear_flash(&session);
    }

    let rendered = tmpl
        .render("index.html.tera", &context)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(rendered))
}
