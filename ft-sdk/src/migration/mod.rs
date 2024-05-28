#[cfg(feature = "sqlite-default")]
mod sqlite;

use diesel::prelude::*;
#[cfg(feature = "sqlite-default")]
use sqlite::MIGRATION_TABLE;

#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
    #[error("Can not create migration table: {0}")]
    CanNotCreateMigrationTable(diesel::result::Error),
    #[error("Can not find latest applied migration number: {0}")]
    CanNotFindLatestAppliedMigrationNumber(diesel::result::Error),
    #[error("Invalid migration: {0}")]
    InvalidMigration(#[from] InvalidMigrationError),
    #[error("apply migration error: {0}")]
    ApplyMigration(#[from] ApplyMigrationError),
}

pub type FnMigration = (
    i32,
    &'static str,
    fn(&mut ft_sdk::Connection) -> Result<(), diesel::result::Error>,
);

pub fn migrate(
    conn: &mut ft_sdk::Connection,
    app_name: &str,
    migration_sqls: include_dir::Dir,
    migration_functions: Vec<FnMigration>,
) -> Result<(), MigrationError> {
    let now = ft_sdk::env::now();
    // check if the migration table exists, if not create it
    create_migration_table(conn).map_err(MigrationError::CanNotCreateMigrationTable)?;

    // Migration for fastn app
    migrate_fastn(conn, now)?;

    // find the latest applied migration number from the table
    migrate_app(conn, app_name, migration_sqls, migration_functions, now)?;

    Ok(())
}

// Migration for fastn app
fn migrate_fastn(
    conn: &mut ft_sdk::Connection,
    now: chrono::DateTime<chrono::Utc>,
) -> Result<(), MigrationError> {
    migrate_app(
        conn,
        "fastn",
        include_dir::include_dir!("$CARGO_MANIFEST_DIR/migrations"),
        vec![],
        now,
    )
}

fn migrate_app(
    conn: &mut ft_sdk::Connection,
    app_name: &str,
    migration_sqls: include_dir::Dir,
    migration_functions: Vec<FnMigration>,
    now: chrono::DateTime<chrono::Utc>,
) -> Result<(), MigrationError> {
    let latest_applied_migration_number = find_latest_applied_migration_number(conn, app_name)
        .map_err(MigrationError::CanNotFindLatestAppliedMigrationNumber)?;

    let migrations = sort_migrations(
        migration_sqls,
        migration_functions,
        latest_applied_migration_number,
    )?;

    for cmd in migrations {
        conn.transaction::<_, ApplyMigrationError, _>(|conn| match cmd {
            Cmd::Sql { id, name, sql } => {
                apply_sql_migration(conn, app_name, id, name.as_str(), sql.as_str(), &now)
            }
            Cmd::Fn { id, name, r#fn } => apply_fn_migration(conn, app_name, id, name, r#fn, &now),
        })?;
    }

    Ok(())
}
#[derive(Debug, thiserror::Error)]
pub enum ApplyMigrationError {
    #[error("failed to apply migration: {0}")]
    ApplyMigration(diesel::result::Error),
    #[error("failed to apply migration: {0}")]
    RecordMigration(diesel::result::Error),
    #[error("failed to commit transaction: {0}")]
    CommitTransaction(#[from] diesel::result::Error),
}

fn apply_sql_migration(
    conn: &mut ft_sdk::Connection,
    app_name: &str,
    id: i32,
    name: &str,
    sql: &str,
    now: &chrono::DateTime<chrono::Utc>,
) -> Result<(), ApplyMigrationError> {
    diesel::connection::SimpleConnection::batch_execute(conn, sql)
        .map_err(ApplyMigrationError::ApplyMigration)?;
    mark_migration_applied(conn, app_name, id, name, now)
        .map_err(ApplyMigrationError::RecordMigration)
}

fn apply_fn_migration(
    conn: &mut ft_sdk::Connection,
    app_name: &str,
    id: i32,
    name: &str,
    r#fn: fn(&mut ft_sdk::Connection) -> Result<(), diesel::result::Error>,
    now: &chrono::DateTime<chrono::Utc>,
) -> Result<(), ApplyMigrationError> {
    r#fn(conn).map_err(ApplyMigrationError::ApplyMigration)?;
    mark_migration_applied(conn, app_name, id, name, now)
        .map_err(ApplyMigrationError::RecordMigration)
}

table! {
    fastn_migration {
        id -> Integer,
        app_name -> Text,
        migration_number -> Integer,
        migration_name -> Text,
        applied_on -> Timestamptz,
    }
}

pub fn mark_migration_applied(
    conn: &mut ft_sdk::Connection,
    app_name: &str,
    id: i32,
    name: &str,
    now: &chrono::DateTime<chrono::Utc>,
) -> Result<(), diesel::result::Error> {
    diesel::insert_into(fastn_migration::table)
        .values((
            fastn_migration::app_name.eq(app_name),
            fastn_migration::migration_number.eq(id),
            fastn_migration::migration_name.eq(name),
            fastn_migration::applied_on.eq(now),
            // fastn_migration::time_taken.eq(time_take),
        ))
        .execute(conn)
        .map(|_| ())
}

fn create_migration_table(conn: &mut ft_sdk::Connection) -> Result<(), diesel::result::Error> {
    diesel::dsl::sql_query(MIGRATION_TABLE).execute(conn)?;
    Ok(())
}

fn find_latest_applied_migration_number(
    conn: &mut ft_sdk::Connection,
    app_name: &str,
) -> Result<Option<i32>, diesel::result::Error> {
    fastn_migration::table
        .select(fastn_migration::migration_number)
        .filter(fastn_migration::app_name.eq(app_name))
        .order(fastn_migration::migration_number.desc())
        .first(conn)
        .optional()
}

#[derive(Debug, thiserror::Error)]
pub enum InvalidMigrationError {
    /// If the same migration exists in both sql and function migrations
    #[error("Duplicate migration number: {0}")]
    DuplicateMigration(i32),
    #[error("Invalid sql file not utf8: {0:?}")]
    InvalidSqlFileNameNotUtf8(std::ffi::OsString),
    #[error("Invalid sql content not utf8: {0}, {1:?}")]
    InvalidSqlFileContentNotUtf8(i32, std::string::FromUtf8Error),
    #[error("SQL file does not start with integer: {0:?}, {1:?}")]
    SqlFileIsNotInteger(String, std::num::ParseIntError),
}

enum Cmd {
    Sql {
        id: i32,
        name: String,
        sql: String,
    },
    Fn {
        id: i32,
        name: &'static str,
        r#fn: fn(&mut ft_sdk::Connection) -> Result<(), diesel::result::Error>,
    },
}

impl Cmd {
    fn id(&self) -> i32 {
        match self {
            Cmd::Sql { id, .. } => *id,
            Cmd::Fn { id, .. } => *id,
        }
    }
}

fn sort_migrations(
    migration_sqls: include_dir::Dir,
    migration_functions: Vec<FnMigration>,
    after: Option<i32>,
) -> Result<Vec<Cmd>, InvalidMigrationError> {
    let mut cmds = vec![];

    for file in migration_sqls.files() {
        if file.path().extension() != Some(std::ffi::OsStr::new("sql")) {
            continue;
        }

        let file_stem = file.path().file_stem().unwrap();
        let file_stem = match file_stem.to_str() {
            Some(v) => v,
            None => {
                return Err(InvalidMigrationError::InvalidSqlFileNameNotUtf8(
                    file_stem.into(),
                ));
            }
        };

        let (migration_number, migration_name) = parse_migration_name(file_stem)?;

        if Some(migration_number) > after {
            cmds.push(Cmd::Sql {
                id: migration_number,
                name: migration_name.to_string(),
                sql: match String::from_utf8(file.contents().to_vec()) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(InvalidMigrationError::InvalidSqlFileContentNotUtf8(
                            migration_number,
                            e,
                        ));
                    }
                },
            })
        }
    }

    for (migration_number, name, the_fn) in migration_functions.into_iter() {
        if Some(migration_number) > after {
            cmds.push(Cmd::Fn {
                id: migration_number,
                name,
                r#fn: the_fn,
            })
        }
    }

    cmds.sort_by_key(|k| k.id());

    Ok(cmds)
}

fn parse_migration_name(name: &str) -> Result<(i32, &str), InvalidMigrationError> {
    let (number, name) = match name.split_once('-') {
        Some(v) => v,
        None => (name, name),
    };

    Ok((
        number
            .parse()
            .map_err(|e| InvalidMigrationError::SqlFileIsNotInteger(name.to_string(), e))?,
        name,
    ))
}
