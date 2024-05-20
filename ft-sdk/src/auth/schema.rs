diesel::table! {
    use diesel::sql_types::*;

    fastn_user (id) {
        id -> Int8,
        name -> Nullable<Text>,
        identity -> Text,
        data -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

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

diesel::joinable!(fastn_session -> fastn_user (uid));

diesel::allow_tables_to_appear_in_same_query!(fastn_user, fastn_session,);
