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
        "/add/" => add(&req),
        "/mark-done/" => mark_done(&req),
        "/delete/" => delete(&req),
        t => ft_sdk::http::not_found(t),
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

fn add(req: &http::Request<bytes::Bytes>) -> http::Response<bytes::Bytes> {
    use ft_sdk::JsonBodyExt;

    let text: String = req.required("text").unwrap();
    let mut conn = ft_sdk::default_connection().unwrap();

    diesel::insert_into(todo_item::table)
        .values((todo_item::text.eq(text), todo_item::is_done.eq(false)))
        .execute(&mut conn)
        .unwrap();

    ft_sdk::http::json("ok").unwrap()
}

fn mark_done(req: &http::Request<bytes::Bytes>) -> http::Response<bytes::Bytes> {
    use ft_sdk::JsonBodyExt;

    let (id, done): (i32, bool) = req.required2("id", "done").unwrap();
    let mut conn = ft_sdk::default_connection().unwrap();

    diesel::update(todo_item::table.find(id))
        .set(todo_item::is_done.eq(done))
        .execute(&mut conn)
        .unwrap();

    ft_sdk::http::json("ok").unwrap()
}

fn delete(req: &http::Request<bytes::Bytes>) -> http::Response<bytes::Bytes> {
    use ft_sdk::JsonBodyExt;

    let id: i32 = req.required("id").unwrap();
    let mut conn = ft_sdk::default_connection().unwrap();
    diesel::delete(todo_item::table.find(id))
        .execute(&mut conn)
        .unwrap();

    ft_sdk::http::json("ok").unwrap()
}
