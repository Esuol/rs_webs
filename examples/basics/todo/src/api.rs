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

#[derive(Debug, Deserialize)]
pub struct CreateForm {
    description: String,
}

pub async fn create(
    params: web::Form<CreateForm>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> Result<impl Responder, Error> {
    if params.description.is_empty() {
        session::set_flash(&session, FlashMessage::error("Description cannot be empty"))?;
        Ok(web::Redirect::to("/").using_status_code(StatusCode::FOUND))
    } else {
        db::create_task(params.into_inner().description, &pool)
            .await
            .map_err(error::ErrorInternalServerError)?;

        session::set_flash(&session, FlashMessage::success("Task successfully created"))?;

        Ok(web::Redirect::to("/").using_status_code(StatusCode::FOUND))
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateParams {
    id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateForm {
    _method: String,
}

pub async fn update(
    db: web::Data<SqlitePool>,
    params: web::Form<UpdateParams>,
    form: web::Form<UpdateForm>,
    session: Session,
) -> Result<impl Responder, Error> {
    Ok(web::Redirect::to(match form._method.as_ref() {
        "put" => toggle(db, params).await?,
        "delete" => delete(db, params, session).await?,
        unsupported_method => {
            let msg = format!("Unsupported HTTP method: {unsupported_method}");
            return Err(error::ErrorBadRequest(msg));
        }
    })
    .using_status_code(StatusCode::FOUND))
}
