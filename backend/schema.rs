// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        created_at -> Nullable<Timestamp>,
        created_by -> Int4,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        name_variable -> Varchar,
        description -> Text,
        price -> Int4,
        quantity -> Int4,
        image_url -> Varchar,
        created_at -> Nullable<Timestamp>,
        created_by -> Int4,
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

diesel::joinable!(categories -> users (created_by));
diesel::joinable!(products -> categories (category_id));
diesel::joinable!(products -> users (created_by));
diesel::joinable!(specifications -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    products,
    specifications,
    users,
);
