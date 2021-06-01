table! {
    buddies (id) {
        id -> Int4,
        uuid -> Varchar,
        name -> Varchar,
        notes -> Text,
        last_contacted -> Varchar,
        birthday -> Nullable<Varchar>,
        location -> Nullable<Varchar>,
        create_timestamp -> Varchar,
        last_update_timestamp -> Varchar,
        delete_timestamp -> Nullable<Varchar>,
        user_uuid -> Varchar,
    }
}

table! {
    interactions (id) {
        id -> Int4,
        uuid -> Varchar,
        notes -> Text,
        participants -> Array<Text>,
        date -> Nullable<Varchar>,
        create_timestamp -> Varchar,
        last_update_timestamp -> Varchar,
        delete_timestamp -> Nullable<Varchar>,
        user_uuid -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        user_id -> Varchar,
        create_timestamp -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    buddies,
    interactions,
    users,
);
