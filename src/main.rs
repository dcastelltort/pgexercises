#[macro_use]
extern crate diesel;
extern crate bigdecimal;
extern crate dotenv;
extern crate chrono;

pub mod schema;
pub mod models;
pub mod utils;
pub mod ex_basic;

fn main() {
    println!("Simple program that implement pgexercices using rust and the diesel framework . USE: cargo test instead");
}


