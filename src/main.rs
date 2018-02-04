#[macro_use]
extern crate diesel;
extern crate bigdecimal;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod utils;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sql_query;

use std::env;
use bigdecimal::BigDecimal;

use models::*;
use utils::*;


fn main() {
    println!("Simple program that implement pgexercices using rust and the diesel framework . USE: cargo test instead");
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

#[test]
fn basic_select_where() {

    use schema::facilities::dsl::*;

    let connection = establish_connection();

    let results_sql : Vec<Facility> = sql_query("SELECT * from facilities WHERE membercost > 0").load::<Facility>(&connection).expect("query failed to run");
    let results : Vec<Facility> = facilities.filter(membercost.gt(BigDecimal::from(0))).get_results::<Facility>(&connection).expect("diesel operation failed");
    
    println!("\nSQL ---------");
    print_results(&results_sql);
    println!("\nDSL ---------");
    print_results(&results);

    assert_eq!(results_sql, results);
}