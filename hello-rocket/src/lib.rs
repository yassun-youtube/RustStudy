#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
// use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;
