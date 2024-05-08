use diesel::prelude::*;

#[ft_sdk::handle_http]
fn handle(in_: ft_sdk::In, mut conn: ft_sdk::Connection) -> ft_sdk::http::Result {
    ft_sdk::migrate_simple!("hello-world", &in_, &mut conn)?;

    match in_.req.uri().path() {
        "/list/" => list(&mut conn),
        "/add/" => add(&in_, &mut conn),
        "/mark-done/" => mark_done(&in_, &mut conn),
        "/delete/" => delete(&in_, &mut conn),
        t => ft_sdk::not_found!("unhandled path: {t}"),
    }
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

/// list() returns list of todos.
///
/// this view should be called from ftd file using http processor
fn list(conn: &mut ft_sdk::Connection) -> ft_sdk::http::Result {
    let items: Vec<TodoItem> = todo_item::table
        .select(TodoItem::as_select())
        .get_results(conn)
        .unwrap();

    ft_sdk::http::json(items)
}

/// add a new item
///
/// this should be called from ftd.http() method from frontend. it tells ftd.js
/// to reload the page after adding the item
fn add(in_: &ft_sdk::In, conn: &mut ft_sdk::Connection) -> ft_sdk::http::Result {
    use ft_sdk::JsonBodyExt;

    let text: String = in_.req.required("text")?;

    diesel::insert_into(todo_item::table)
        .values((todo_item::text.eq(text), todo_item::is_done.eq(false)))
        .execute(conn)?;

    ft_sdk::http::reload()
}

/// mark an item as done
///
/// this should be called from ftd.http() method from frontend. it tells ftd.js
/// to reload the page after updating the item
fn mark_done(in_: &ft_sdk::In, conn: &mut ft_sdk::Connection) -> ft_sdk::http::Result {
    use ft_sdk::JsonBodyExt;

    let (id, done): (i32, bool) = in_.req.required2("id", "done").unwrap();

    diesel::update(todo_item::table.find(id))
        .set(todo_item::is_done.eq(done))
        .execute(conn)
        .unwrap();

    ft_sdk::http::reload()
}

/// delete an item
///
/// this should be called from ftd.http() method from frontend. it tells ftd.js
/// to reload the page after deleting the item
fn delete(in_: &ft_sdk::In, conn: &mut ft_sdk::Connection) -> ft_sdk::http::Result {
    use ft_sdk::JsonBodyExt;

    let id: i32 = in_.req.required("id").unwrap();
    diesel::delete(todo_item::table.find(id))
        .execute(conn)
        .unwrap();

    ft_sdk::http::reload()
}
