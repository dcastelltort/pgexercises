use super::schema::{bookings, facilities, members};
use std::time::SystemTime;
use bigdecimal::BigDecimal;

#[derive(Queryable,Identifiable,AsChangeset,Debug)]
#[primary_key(bookid)]
pub struct Booking {
    bookid : i32,
    facid : i32,
    memid : i32,
    starttime : SystemTime,
    slots : i32
}

#[derive(Queryable,Identifiable,AsChangeset,Debug, QueryableByName, PartialEq)]
#[primary_key(facid)]
#[table_name = "facilities"]
pub struct Facility {
    facid : i32,
    name : String,
    membercost : BigDecimal,
    guestcost : BigDecimal,
    initialoutlay : BigDecimal,
    monthlymaintenance : BigDecimal
}

#[derive(Queryable,Identifiable,AsChangeset,Debug)]
#[primary_key(memid)]
pub struct Member {
    memid : i32,
    surname : String,
    firstname : String,
    address : String,
    zipcode :i32,
    telephone : String,
    recommendedby : Option<i32>,
    joindate : SystemTime
}