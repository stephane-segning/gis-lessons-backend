use diesel::table;

table! {
    activities {
        id -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        meta -> Nullable<Jsonb>,

        user_id -> Text,
        entity_id -> Text,
        entity_type -> VarChar,
        content -> VarChar,
        action_type -> VarChar
    }
}

table! {
    courses {
        id -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        meta -> Nullable<Jsonb>,

        name -> Text,
        slug -> Text,
        description -> Nullable<Text>,
    }
}
