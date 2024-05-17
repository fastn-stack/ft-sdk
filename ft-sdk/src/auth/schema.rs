diesel::table! {
    use diesel::sql_types::*;

    fastn_user (id) {
        id -> Int8,
        name -> Nullable<Text>,
        identity -> Text,
        data -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    fastn_session (id) {
        id -> Text,
        uid -> Nullable<Int8>,
        data -> Jsonb,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}
