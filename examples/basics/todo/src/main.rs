use std::{env, io};

use actix_files::Files;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    http,
    middleware::{ErrorHandlers, Logger},
    web, App, HttpServer,
};
use dotenvy::dotenv;
use tera::Tera;

mod api;
mod db;
mod model;
mod session;

static SESSION_SIGNING_KEY: &[u8] = &[0; 64];

fn main() {
    println!("Hello, world!");
}
