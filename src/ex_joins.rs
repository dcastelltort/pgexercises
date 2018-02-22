use diesel::prelude::*;
use diesel::sql_query;

use chrono::NaiveDateTime;

use models::*;
use utils::*;

///Joins Simplejoin
pub fn joins_simplejoin() -> (Vec<String>, Vec<String>) {
    use schema::members::dsl::*;
    use schema::bookings::dsl::*;

    let connection = establish_connection();

    //SQL
    let results_sql: Vec<String> = sql_query(
        "SELECT starttime FROM bookings bks \
         INNER JOIN members mems ON bks.memid = mems.memid \
         WHERE mems.firstname = 'David' and mems.surname = 'Farrell'",
    ).load::<Booking1>(&connection)
        .expect("query failed to run")
        .into_iter()
        .map(|booking: Booking1| booking.starttime.format("%Y-%m-%d %H:%M:%S").to_string())
        .collect();

    //DSL
    let results: Vec<String> = bookings
        .inner_join(members) //must be declared joinable in schema first
        .select(starttime)
        .filter(firstname.eq("David"))
        .filter(surname.eq("Farrell"))
        .load::<NaiveDateTime>(&connection)
        .expect("diesel operation failed")
        .into_iter()
        .map(|date: NaiveDateTime| { date.format("%Y-%m-%d %H:%M:%S").to_string()})
        .collect();

    (results_sql, results)
}

#[test]
fn test_joins_simplejoin() {
    test_results(&joins_simplejoin);
}
