
use diesel::prelude::*;
use diesel::sql_query;

use bigdecimal::BigDecimal;

use models::*;
use utils::*;

pub fn basic_selectall() -> (Vec<Facility>, Vec<Facility> ) {
    
    let connection = establish_connection();

    use schema::facilities::dsl::*;

    let results_sql : Vec<Facility> = sql_query("SELECT * from facilities")
                                        .load(&connection)
                                        .expect("query failed to run");
    let results : Vec<Facility> = facilities.load(&connection)
                                            .expect("diesel operation failed");
    
    (results_sql, results)
}

#[test]
fn test_basic_selectall() {

    let (results_sql, results) = basic_selectall();

    println!("\nSQL ---------");
    print_results(&results_sql);
    println!("\nDSL ---------");
    print_results(&results);

    assert_eq!(results_sql, results);
}

pub fn basic_select_specific() -> (Vec<FacilityPartial>, Vec<FacilityPartial>) {
    use schema::facilities::dsl::*;

    let connection = establish_connection();

    let results_sql : Vec<FacilityPartial> = sql_query("SELECT name, membercost FROM facilities")
                                                .load::<FacilityPartial>(&connection)
                                                .expect("query failed to run");
    let results : Vec<FacilityPartial> = facilities.select((name, membercost))
                                                    .load::<FacilityPartial>(&connection)
                                                    .expect("diesel operation failed");
    
    (results_sql, results)   
}

#[test]
fn test_basic_select_specific()  {
    
    let (results_sql , results) = basic_select_specific();

    println!("\nSQL ---------");
    print_results(&results_sql);
    println!("\nDSL ---------");
    print_results(&results);

    assert_eq!(results_sql, results);
}

pub fn basic_select_where() -> (Vec<Facility> , Vec<Facility>) {

    use schema::facilities::dsl::*;

    let connection = establish_connection();

    let results_sql : Vec<Facility> = sql_query("SELECT * from facilities WHERE membercost > 0")
                                        .load::<Facility>(&connection)
                                        .expect("query failed to run");
    let results : Vec<Facility> = facilities.filter(membercost.gt(BigDecimal::from(0)))
                                            .get_results::<Facility>(&connection)
                                            .expect("diesel operation failed");
    
    (results_sql, results)
}

#[test]
fn test_basic_select_where() {
    
    let (results_sql, results) = basic_select_where();

    println!("\nSQL ---------");
    print_results(&results_sql);
    println!("\nDSL ---------");
    print_results(&results);

    assert_eq!(results_sql, results);
}