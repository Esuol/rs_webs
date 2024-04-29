use std::{convert::Infallible, io};

use actix_files::{Files, NamedFile};
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{
    error, get,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
    middleware, web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use async_stream::stream;

static SESSION_SIGNING_KEY: &[u8] = &[0; 64];

#[get("/favicon")]
async fn favicon() -> Result<impl Responder> {
    Ok(NamedFile::open("static/favicon")?)
}

#[get("/welcome")]
async fn welcome(req: HttpRequest, session: Session) -> Result<HttpResponse> {
    println!("{req:?}");

    // session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {count}");
        counter = count + 1;
    }
    // set counter to session
    session.insert("counter", counter)?;

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(include_str!("../static/welcome.html")))
}

async fn default_handler(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => {
            let file = NamedFile::open("static/404.html")?
                .customize()
                .with_status(StatusCode::NOT_FOUND);
            Ok(Either::Left(file))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}

async fn response_body(path: web::Path<String>) -> HttpResponse {
    let name = path.into_inner();

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .streaming(stream! {
            yield Ok::<_, Infallible>(web::Bytes::from("Hello "));
            yield Ok::<_, Infallible>(web::Bytes::from(name));
            yield Ok::<_, Infallible>(web::Bytes::from("!"));
        })
}

async fn with_param(req: HttpRequest, path: web::Path<(String,)>) -> HttpResponse {
    println!("{req:?}");

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(format!("Hello {}!", path.0))
}

fn main() {
    println!("Hello, world!");
}
