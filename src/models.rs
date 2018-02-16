use super::schema::{bookings, facilities, members};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

/// Booking models
#[derive(Queryable, Identifiable, AsChangeset, Debug)]
#[primary_key(bookid)]
pub struct Booking {
    bookid: i32,
    facid: i32,
    memid: i32,
    starttime: NaiveDateTime,
    slots: i32,
}

/// Facility Models
#[derive(Queryable, Identifiable, AsChangeset, Debug, QueryableByName, PartialEq)]
#[primary_key(facid)]
#[table_name = "facilities"]
pub struct Facility {
    facid: i32,
    name: String,
    membercost: BigDecimal,
    guestcost: BigDecimal,
    initialoutlay: BigDecimal,
    monthlymaintenance: BigDecimal,
}

#[derive(Queryable, PartialEq, Debug, QueryableByName)]
#[table_name = "facilities"]
pub struct FacilityPartial {
    name: String,
    membercost: BigDecimal,
}

#[derive(Queryable, PartialEq, Debug, QueryableByName)]
#[table_name = "facilities"]
pub struct FacilityPartial1 {
    pub name: String,
}

#[derive(Queryable, PartialEq, Debug, QueryableByName)]
#[table_name = "facilities"]
pub struct FacilityPartial2 {
    pub name: String,
    pub monthlymaintenance: BigDecimal,
}

#[derive(Queryable, Identifiable, PartialEq, Debug, QueryableByName)]
#[primary_key(facid)]
#[table_name = "facilities"]
pub struct FacilityPartial4 {
    facid: i32,
    name: String,
    membercost: BigDecimal,
    monthlymaintenance: BigDecimal,
}

/// Members Models
#[derive(Queryable, Identifiable, AsChangeset, Debug)]
#[primary_key(memid)]
pub struct Member {
    memid: i32,
    surname: String,
    firstname: String,
    address: String,
    zipcode: i32,
    telephone: String,
    recommendedby: Option<i32>,
    joindate: NaiveDateTime,
}

#[derive(Queryable, Debug, QueryableByName, PartialEq)]
#[table_name = "members"]
pub struct Member1 {
    pub surname: String,
}

#[derive(Queryable, Debug, QueryableByName, PartialEq)]
#[table_name = "members"]
pub struct Member1jd {
    pub joindate: NaiveDateTime,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, QueryableByName, PartialEq)]
#[primary_key(memid)]
#[table_name = "members"]
pub struct Member2 {
    memid: i32,
    surname: String,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, QueryableByName, PartialEq)]
#[primary_key(memid)]
#[table_name = "members"]
pub struct Member2jd {
    memid: i32,
    pub joindate: NaiveDateTime,
}

#[derive(Queryable, Debug, QueryableByName, PartialEq)]
#[table_name = "members"]
pub struct Member3 {
    firstname: String,
    surname: String,
    joindate: NaiveDateTime,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, QueryableByName, PartialEq)]
#[primary_key(memid)]
#[table_name = "members"]
pub struct Member4 {
    memid: i32,
    surname: String,
    firstname: String,
    joindate: NaiveDateTime,
}
