// @generated automatically by Diesel CLI.

diesel::table! {
    product (id) {
        id -> Int4,
        name -> Text,
        inventory -> Int4,
        category -> Text,
        tags -> Nullable<Text>,
        keywords -> Nullable<Text>,
        thumbnail_url -> Nullable<Text>,
        gallery_urls -> Nullable<Text>,
        tagline -> Nullable<Text>,
        description -> Nullable<Text>,
        discount_percent -> Nullable<Float4>,
        added_date -> Timestamp,
        restock_date -> Nullable<Timestamp>,
        price -> Int8,
    }
}

diesel::table! {
    session (session_id) {
        session_id -> Uuid,
        user_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        expires_at -> Timestamp,
        ip_address -> Nullable<Text>,
        user_agent -> Nullable<Text>,
        cart_data -> Nullable<Jsonb>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    product,
    session,
);
