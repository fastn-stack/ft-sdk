use diesel::prelude::*;

#[no_mangle]
pub extern "C" fn main_ft() {
    ft_sdk::migrate_simple(
        "hello-world",
        include_dir::include_dir!("$CARGO_MANIFEST_DIR/migrations"),
    )
    .unwrap();

    let req = ft_sdk::http::current_request();

    ft_sdk::http::send_response(match req.uri().path() {
        "/list/" => list(),
        t => http::Response::builder()
            .status(200)
            .body(format!("page not found: {t}\n").into())
            .unwrap(),
    })
}

table! {
    todo_item {
        id -> Integer,
        text -> Text,
        is_done -> Bool,
    }
}

#[derive(diesel::Queryable, diesel::Selectable, Debug, serde::Serialize)]
#[diesel(table_name = todo_item)]
struct TodoItem {
    id: i32,
    text: String,
    is_done: bool,
}

fn list() -> http::Response<bytes::Bytes> {
    let mut conn = ft_sdk::default_connection().unwrap();

    let items: Vec<TodoItem> = todo_item::table
        .select(TodoItem::as_select())
        .get_results(&mut conn)
        .unwrap();

    ft_sdk::http::json(items).unwrap()
}
