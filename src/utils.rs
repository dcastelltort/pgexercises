
use std::env;
use std::fmt;

use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn print_results<T>(results: &Vec<T>) where T : fmt::Debug {
    for r in results {
        println!("{:?}", r);
    }
}