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

fn main() {
    println!("Hello, world!");
}
