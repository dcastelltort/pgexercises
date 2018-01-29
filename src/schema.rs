table! {
    bookings (bookid) {
        bookid -> Int4,
        facid -> Int4,
        memid -> Int4,
        starttime -> Timestamp,
        slots -> Int4,
    }
}

table! {
    facilities (facid) {
        facid -> Int4,
        name -> Varchar,
        membercost -> Numeric,
        guestcost -> Numeric,
        initialoutlay -> Numeric,
        monthlymaintenance -> Numeric,
    }
}

table! {
    members (memid) {
        memid -> Int4,
        surname -> Varchar,
        firstname -> Varchar,
        address -> Varchar,
        zipcode -> Int4,
        telephone -> Varchar,
        recommendedby -> Nullable<Int4>,
        joindate -> Timestamp,
    }
}

joinable!(bookings -> facilities (facid));
joinable!(bookings -> members (memid));

allow_tables_to_appear_in_same_query!(
    bookings,
    facilities,
    members,
);