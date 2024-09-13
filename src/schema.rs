// @generated automatically by Diesel CLI.

diesel::table! {
    rustacs (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}
