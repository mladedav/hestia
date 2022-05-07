table! {
    recipes (id) {
        id -> Integer,
        title -> Text,
        content -> Nullable<Text>,
        ingredients -> Nullable<Text>,
        tips -> Nullable<Text>,
        picture -> Nullable<Text>,
        preparation_minutes -> Nullable<Integer>,
        stars -> Integer,
        class -> Nullable<Text>,
        tags -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password_hash -> Binary,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    recipes,
    users,
);
