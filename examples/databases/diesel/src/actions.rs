use diesel::prelude::*;
use uuid::uuid;

use crate::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;
