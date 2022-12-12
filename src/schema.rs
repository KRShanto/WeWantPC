// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        role -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}
