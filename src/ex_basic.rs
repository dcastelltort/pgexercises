
use diesel::prelude::*;
use diesel::sql_query;

use bigdecimal::BigDecimal;
use chrono::{NaiveDate};

use models::*;
use utils::*;

/// Basic Select All
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

// Basic Select Specific
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

/// Basic Select Where
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

/// Basic Select Where2
pub fn basic_select_where2() -> (Vec<FacilityPartial4> , Vec<FacilityPartial4>) {

    use schema::facilities::dsl::*;

    let connection = establish_connection();

    let results_sql : Vec<FacilityPartial4> = sql_query("SELECT facid, name, membercost, monthlymaintenance FROM facilities WHERE membercost > 0 AND membercost < monthlymaintenance/50;")
                                        .load::<FacilityPartial4>(&connection)
                                        .expect("query failed to run");
    let results : Vec<FacilityPartial4> = facilities.select((facid, name, membercost, monthlymaintenance))
                                            .filter(membercost.gt(BigDecimal::from(0)))
                                            .filter(membercost.lt(monthlymaintenance / BigDecimal::from(50))) 
                                            .get_results::<FacilityPartial4>(&connection)
                                            .expect("diesel operation failed");
    
    (results_sql, results)
}

#[test]
fn test_basic_select_where2() {
    
    let (results_sql, results) = basic_select_where2();

    println!("\nSQL ---------");
    print_results(&results_sql);
    println!("\nDSL ---------");
    print_results(&results);

    assert_eq!(results_sql, results);
}

/// Basic Where 3
pub fn basic_select_where3() -> (Vec<Facility>, Vec<Facility>) {

    use schema::facilities::dsl::*;

    let connection = establish_connection();

    let results_sql : Vec<Facility> = sql_query("SELECT * FROM facilities WHERE name LIKE '%Tennis%'")
                                        .load::<Facility>(&connection)
                                        .expect("query failed to run");
    let results : Vec<Facility> = facilities.filter(name.like("%Tennis%")) 
                                            .get_results::<Facility>(&connection)
                                            .expect("diesel operation failed");
    
    (results_sql, results)
}

#[test]
fn test_basic_select_where3() {
    let (results_sql, results) = basic_select_where3();

    println!("\nSQL ---------");
    print_results(&results_sql);
    println!("\nDSL ---------");
    print_results(&results);

    assert_eq!(results_sql, results);
}



/// Basic Where 4
pub fn basic_select_where4() -> (Vec<Facility>, Vec<Facility>) {

    use schema::facilities::dsl::*;

    let connection = establish_connection();

    let results_sql : Vec<Facility> = sql_query("SELECT * FROM facilities WHERE facid IN (1,5)")
                                        .load::<Facility>(&connection)
                                        .expect("query failed to run");
    let results : Vec<Facility> = facilities.filter(facid.eq_any(vec![1,5]))
                                            .get_results::<Facility>(&connection)
                                            .expect("diesel operation failed");
    
    (results_sql, results)
}

#[test]
fn test_basic_select_where4() {
    let (results_sql, results) = basic_select_where4();

    println!("\nSQL ---------");
    print_results(&results_sql);
    println!("\nDSL ---------");
    print_results(&results);

    assert_eq!(results_sql, results);
}

///Basic Classify
pub fn basic_classify() -> (Vec<(String, String)>, Vec<(String, String)>) {
    use schema::facilities::dsl::*;

    let connection = establish_connection();
    // trying to do the same as 
    // "SELECT name,
	// CASE WHEN monthlymaintenance < 100 THEN 'cheap' ELSE 'expensive' END AS cost
	// FROM facilities"
    // 
    let intermediate_sql : Vec<FacilityPartial2> = sql_query("SELECT name,
	                                            monthlymaintenance FROM facilities")
                                        .load::<FacilityPartial2>(&connection)
                                        .expect("query failed to run");
    let intermediate_results : Vec<(String,BigDecimal)> = facilities.select((name, monthlymaintenance))
                                            .load::<(String,BigDecimal)>(&connection)
                                            .expect("diesel operation failed");
    
    let results_sql : Vec<(String, String)> = intermediate_sql.into_iter().map(|res: FacilityPartial2| {
        let cost = if res.monthlymaintenance < BigDecimal::from(100) {String::from("cheap")} else {String::from("expensive")};
        (res.name, cost)
    }).collect();

    let results : Vec<(String, String)> = intermediate_results.into_iter().map(|res: (String,BigDecimal)| {
        let cost = if res.1 < BigDecimal::from(100) {String::from("cheap")} else {String::from("expensive")};
        (res.0, cost)
    }).collect();

    (results_sql, results)
}

#[test]
fn test_basic_classify() {
    let (results_sql, results) = basic_classify();

    println!("\nSQL ---------");
    print_results(&results_sql);
    println!("\nDSL ---------");
    print_results(&results);

    assert_eq!(results_sql, results);
}

/// Basic Date
pub fn basic_date() -> (Vec<Member4>, Vec<Member4>) {

    use schema::members::dsl::*;

    let connection = establish_connection();

    let results_sql : Vec<Member4> = sql_query("SELECT memid, surname, firstname, joindate FROM members WHERE joindate > TIMESTAMP '2012-09-01'")
                                        .load::<Member4>(&connection)
                                        .expect("query failed to run");
    let results : Vec<Member4> = members.select((memid, surname, firstname, joindate))
                                        .filter(joindate.gt(NaiveDate::from_ymd(2012, 9, 1).and_hms(0,0,0)))
                                        .get_results::<Member4>(&connection)
                                        .expect("diesel operation failed");
    
    (results_sql, results)
}

#[test]
fn test_basic_date() {
    let (results_sql, results) = basic_date();

    println!("\nSQL ---------");
    print_results(&results_sql);
    println!("\nDSL ---------");
    print_results(&results);

    assert_eq!(results_sql, results);
}