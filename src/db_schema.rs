table! {
    fragments (id) {
        id -> Uuid,
        schema_path -> Text,
        text_id -> Nullable<Uuid>,
        created_at -> Timestamptz,
    }
}

table! {
    schemas (id) {
        id -> Uuid,
        text_id -> Uuid,
        paths -> Array<Text>,
        created_at -> Timestamptz,
    }
}

table! {
    texts (id) {
        id -> Uuid,
        title -> Text,
        slug -> Varchar,
        authors -> Varchar,
        description -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(fragments -> texts (text_id));
joinable!(schemas -> texts (text_id));

allow_tables_to_appear_in_same_query!(
    fragments,
    schemas,
    texts,
);
