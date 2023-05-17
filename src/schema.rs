// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        username -> Nullable<Text>,
        email -> Nullable<Text>,
        img_url -> Nullable<Text>,
        phone -> Nullable<Double>,
        password -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}
