table! {
    buddies (id) {
        id -> Int4,
        uuid -> Varchar,
        name -> Varchar,
        user_id -> Varchar,
        notes -> Text,
	last_contacted -> Varchar,
	birthday -> Nullable<Varchar>,
	location -> Nullable<Varchar>,
        create_timestamp -> Varchar,
        last_update_timestamp -> Varchar,
        delete_timestamp -> Nullable<Varchar>,
    }
}
