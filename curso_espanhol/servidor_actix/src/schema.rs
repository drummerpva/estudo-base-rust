// @generated automatically by Diesel CLI.

diesel::table! {
    likes (id) {
        id -> Unsigned<Integer>,
        created_at -> Timestamp,
        tweet_id -> Nullable<Unsigned<Integer>>,
    }
}

diesel::table! {
    tweets (id) {
        id -> Unsigned<Integer>,
        created_at -> Timestamp,
        message -> Text,
    }
}

diesel::joinable!(likes -> tweets (tweet_id));

diesel::allow_tables_to_appear_in_same_query!(
    likes,
    tweets,
);
