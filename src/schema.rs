// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        username -> Text,
        email -> Nullable<Text>,
        img_url -> Nullable<Text>,
        phone -> Nullable<Double>,
        password -> Text,
        created_at -> Timestamp,
    }
}
