#[macro_use]
extern crate diesel;
extern crate bigdecimal;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sql_query;
use dotenv::dotenv;
use std::env;


use models::*;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}


fn main() {
    println!("Simple program that implement pgexercices using rust and the diesel framework . USE: cargo test instead");
}

fn print_results<T>(results: &Vec<T>) where T : std::fmt::Debug {
    for r in results {
        println!("{:?}", r);
    }
}

#[test]
fn basic_selectall() {
    
    let connection = establish_connection();

    use schema::facilities::dsl::*;

    let results_sql : Vec<Facility> = sql_query("SELECT * from facilities").load(&connection).expect("query failed to run");
    let results : Vec<Facility> = facilities.load(&connection).expect("diesel operation failed");
    
    println!("\nSQL ---------");
    print_results(&results_sql);
    println!("\nDSL ---------");
    print_results(&results);

    assert_eq!(results_sql, results);
}

#[test]
fn basic_select_specific() {

    use schema::facilities::dsl::*;

    let connection = establish_connection();

    let results_sql : Vec<FacilityPartial> = sql_query("SELECT name, membercost FROM facilities").load::<FacilityPartial>(&connection).expect("query failed to run");
    let results : Vec<FacilityPartial> = facilities.select((name, membercost)).load::<FacilityPartial>(&connection).expect("diesel operation failed");
    
    println!("\nSQL ---------");
    print_results(&results_sql);
    println!("\nDSL ---------");
    print_results(&results);

    assert_eq!(results_sql, results);
}