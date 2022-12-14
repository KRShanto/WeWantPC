// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        name_variable -> Varchar,
        description -> Text,
        price -> Int4,
        quantity -> Int4,
        image_url -> Varchar,
        createdat -> Nullable<Timestamp>,
        category_id -> Int4,
        brand -> Varchar,
    }
}

diesel::table! {
    specifications (id) {
        id -> Int4,
        names_values -> Nullable<Jsonb>,
        product_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        role -> Varchar,
        created_at -> Nullable<Timestamp>,
        address -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        verified -> Nullable<Bool>,
        img_url -> Nullable<Varchar>,
    }
}

diesel::joinable!(specifications -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    products,
    specifications,
    users,
);
