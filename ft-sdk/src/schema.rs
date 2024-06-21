diesel::table! {
    use diesel::sql_types::*;

    fastn_session (id) {
        id -> Text,
        uid -> Nullable<Int8>,
        data -> Text,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}
