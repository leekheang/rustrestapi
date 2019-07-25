table! {
    roles (id) {
        id -> Int4,
        title -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        avatar -> Nullable<Varchar>,
        biography -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    roles,
    users,
);
