use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Product {
    id: Option<i64>,
    product_type: Option<String>,
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Part {
    id: Option<i64>,
    part_type: Option<String>,
    name: Option<String>,
}
