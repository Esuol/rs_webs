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
}

fn main() {
    println!("Hello, world!");
}
