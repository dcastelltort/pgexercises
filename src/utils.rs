
use std::env;
use std::fmt;

use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;

/// utiliy function to connect to Postgres
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

/// utility function used to print results from plain SQL and DSL queries
pub fn print_results<T>(results: &Vec<T>) where T : fmt::Debug {
    for r in results {
        println!("{:?}", r);
    }
}

pub fn test_results<T>(f: &Fn() ->(Vec<T>,Vec<T>)) where T: fmt::Debug + PartialEq {

    let (results_sql, results) = f();

    println!("\nSQL ---------");
    print_results(&results_sql);
    println!("\nDSL ---------");
    print_results(&results);

    assert_eq!(results_sql, results);
}
