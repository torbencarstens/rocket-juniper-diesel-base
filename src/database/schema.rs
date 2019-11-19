table! {
    events (id) {
        id -> Int4,
        timestamp -> Timestamp,
        location -> Varchar,
    }
}

table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        age -> Int2,
    }
}

allow_tables_to_appear_in_same_query!(
    events,
    groups,
    users,
);
