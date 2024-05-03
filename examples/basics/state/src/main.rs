use std::{
    cell::Cell,
    io,
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

fn main() {
    println!("Hello, world!");
}
