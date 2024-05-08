use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .data_factory(|| async { Ok::<_, ()>(123_usize) })
            .route(
                "/",
                web::to(|data: web::Data<usize>| async move {
                    assert_eq!(**data, 123);
                    HttpResponse::NoContent().finish()
                }),
            )
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
