use std::{
    cell::Cell,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
};

use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer,
};

async fn index(
    counter_mutex: Data<Mutex<usize>>,
    counter_cell: Data<Cell<u32>>,
    counter_atomic: Data<AtomicUsize>,
    req: HttpRequest,
) -> HttpResponse {
    println!("{req:?}");

    *counter_mutex.lock().unwrap() += 1;
    counter_cell.set(counter_cell.get() + 1);
    counter_atomic.fetch_add(1, Ordering::SeqCst);

    let body = format!(
        "global mutex counter: {}, local counter: {}, global atomic counter: {}",
        *counter_mutex.lock().unwrap(),
        counter_cell.get(),
        counter_atomic.load(Ordering::SeqCst),
    );

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    #[allow(clippy::mutex_atomic)] // it's intentional.
    let counter_mutex = Data::new(Mutex::new(0usize));
    let counter_atomic = Data::new(AtomicUsize::new(0usize));

    log::info!("starting HTTP server at http://localhost:8080");

    // move is necessary to give closure below ownership of counter1
    HttpServer::new(move || {
        // Create some thread-local state
        let counter_cell = Cell::new(0u32);

        App::new()
            .app_data(counter_mutex.clone()) // add shared state
            .app_data(counter_atomic.clone()) // add shared state
            .app_data(Data::new(counter_cell)) // add thread-local state
            // enable logger
            .wrap(middleware::Logger::default())
            // register simple handler
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
