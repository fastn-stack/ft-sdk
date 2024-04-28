#[cfg(feature = "sqlite-default")]
mod sqlite;

use diesel::prelude::*;
#[cfg(feature = "sqlite-default")]
use sqlite::{EMAIL_TABLE, MIGRATION_TABLE, SESSION_TABLE, USER_TABLE};

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

pub fn migrate<T>(
    conn: &mut ft_sdk::Connection,
    migration_sqls: include_dir::Dir,
    migration_functions: std::collections::HashMap<i32, T>,
) -> Result<(), MigrationError>
where
    T: FnOnce(&mut ft_sdk::Connection) -> Result<(), diesel::result::Error>,
{
    // check if the migration table exists, if not create it
    create_migration_table(conn).map_err(MigrationError::CanNotCreateMigrationTable)?;

    // find the latest applied migration number from the table
    let latest_applied_migration_number = find_latest_applied_migration_number(conn)
        .map_err(MigrationError::CanNotFindLatestAppliedMigrationNumber)?;

    let migrations = sort_migrations(
        migration_sqls,
        migration_functions,
        latest_applied_migration_number,
    )?;

    for migration in migrations {
        match migration {
            Cmd::Sql { id, sql } => apply_sql_migration(conn, id, sql.as_str())?,
            Cmd::Fn { id, r#fn } => apply_fn_migration(conn, id, r#fn)?,
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum ApplyMigrationError {
    #[error("failed to apply migration: {0}")]
    FailedToApplyMigration(diesel::result::Error),
    #[error("failed to apply migration: {0}")]
    FailedToRecordMigration(diesel::result::Error),
}

fn apply_sql_migration(
    conn: &mut ft_sdk::Connection,
    id: i32,
    sql: &str,
) -> Result<(), ApplyMigrationError> {
    diesel::dsl::sql_query(sql)
        .execute(conn)
        .map_err(ApplyMigrationError::FailedToApplyMigration)?;
    mark_migration_applied(conn, id).map_err(ApplyMigrationError::FailedToRecordMigration)
}

fn apply_fn_migration<T>(
    conn: &mut ft_sdk::Connection,
    id: i32,
    r#fn: T,
) -> Result<(), ApplyMigrationError>
where
    T: FnOnce(&mut ft_sdk::Connection) -> Result<(), diesel::result::Error>,
{
    r#fn(conn).map_err(ApplyMigrationError::FailedToApplyMigration)?;
    mark_migration_applied(conn, id).map_err(ApplyMigrationError::FailedToRecordMigration)
}

table! {
    fastn_migration {
        id -> Integer,
        migration_number -> Integer,
        applied_on -> Timestamptz,
    }
}

pub fn mark_migration_applied(
    conn: &mut ft_sdk::Connection,
    id: i32,
) -> Result<(), diesel::result::Error> {
    diesel::insert_into(fastn_migration::table)
        .values(fastn_migration::migration_number.eq(id))
        .execute(conn)
        .map(|_| ())
}

fn create_migration_table(conn: &mut ft_sdk::Connection) -> Result<(), diesel::result::Error> {
    diesel::dsl::sql_query(MIGRATION_TABLE).execute(conn)?;
    diesel::dsl::sql_query(EMAIL_TABLE).execute(conn)?;
    diesel::dsl::sql_query(USER_TABLE).execute(conn)?;
    diesel::dsl::sql_query(SESSION_TABLE).execute(conn)?;
    Ok(())
}

fn find_latest_applied_migration_number(
    conn: &mut ft_sdk::Connection,
) -> Result<Option<i32>, diesel::result::Error> {
    fastn_migration::table
        .select(fastn_migration::migration_number)
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
    #[error("SQL file is not integer: {0:?}")]
    SqlFileIsNotInteger(#[from] std::num::ParseIntError),
}

enum Cmd<T> {
    Sql { id: i32, sql: String },
    Fn { id: i32, r#fn: T },
}

impl<T> Cmd<T> {
    fn id(&self) -> i32 {
        match self {
            Cmd::Sql { id, .. } => *id,
            Cmd::Fn { id, .. } => *id,
        }
    }
}

fn sort_migrations<T>(
    migration_sqls: include_dir::Dir,
    migration_functions: std::collections::HashMap<i32, T>,
    after: Option<i32>,
) -> Result<Vec<Cmd<T>>, InvalidMigrationError>
where
    T: FnOnce(&mut ft_sdk::Connection) -> Result<(), diesel::result::Error>,
{
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
                ))
            }
        };

        let migration_number = file_stem.parse()?;

        if migration_functions.contains_key(&migration_number) {
            return Err(InvalidMigrationError::DuplicateMigration(migration_number));
        }

        if Some(migration_number) > after {
            cmds.push(Cmd::Sql {
                id: migration_number,
                sql: match String::from_utf8(file.contents().to_vec()) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(InvalidMigrationError::InvalidSqlFileContentNotUtf8(
                            migration_number,
                            e,
                        ))
                    }
                },
            })
        }
    }

    for (migration_number, the_fn) in migration_functions.into_iter() {
        if Some(migration_number) > after {
            cmds.push(Cmd::Fn {
                id: migration_number,
                r#fn: the_fn,
            })
        }
    }

    cmds.sort_by_key(|k| k.id());

    Ok(cmds)
}
