// @generated automatically by Diesel CLI.

diesel::table! {
    product (id) {
        id -> Int4,
        name -> Text,
        price -> Numeric,
        inventory -> Int4,
        category -> Text,
        tags -> Nullable<Text>,
        keywords -> Nullable<Text>,
        thumbnail_url -> Nullable<Text>,
        gallery_urls -> Nullable<Text>,
        tagline -> Nullable<Text>,
        description -> Nullable<Text>,
        discount_percent -> Nullable<Float4>,
    }
}
