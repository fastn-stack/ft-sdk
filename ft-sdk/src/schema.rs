diesel::table! {
    use diesel::sql_types::*;

    fastn_tracker (id) {
        id -> Text,
        uid -> Nullable<Int8>,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(fastn_tracker -> ft_sdk::auth::fastn_user (uid));
