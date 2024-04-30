use actix_web::{web, Error, HttpResponse};

use crate::common::{Part, Product};

pub async fn get_products(_query: web::Query<Option<Part>>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn add_product(_new_product: web::Json<Product>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_product_detail(_id: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn remove_product(_id: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}


#[cfg(test)]
mod tests {
  use actix_web::{
    dev::Service,
    http::{header, StatusCode},
    test, App
  };
}