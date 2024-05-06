use actix_files::NamedFile;
use actix_session::Session;
use actix_web::{
    dev, error, http::StatusCode, middleware::ErrorHandlerResponse, web, Error, HttpResponse,
    Responder, Result,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use tera::{Context, Tera};

