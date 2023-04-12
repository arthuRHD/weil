// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        email -> Varchar,
    }
}
