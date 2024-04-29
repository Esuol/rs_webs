use actix_web::{web, App, Error, HttpResponse, HttpServer, ResponseError};
use derive_more::Display;
use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};

#[derive(Debug, Display)]
pub enum CustomError {
    #[display(fmt = "Custom Error 1")]
    CustomOne,
    #[display(fmt = "Custom Error 2")]
    CustomTwo,
    #[display(fmt = "Custom Error 3")]
    CustomThree,
    #[display(fmt = "Custom Error 4")]
    CustomFour,
}

impl Distribution<CustomError> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CustomError {
        match rng.gen_range(0..4) {
            0 => CustomError::CustomOne,
            1 => CustomError::CustomTwo,
            2 => CustomError::CustomThree,
            _ => CustomError::CustomFour,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
