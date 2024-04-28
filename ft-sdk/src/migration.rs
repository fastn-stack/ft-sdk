use diesel::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
    #[error("Can not create migration table: {0}")]
    CanNotCreateMigrationTable(diesel::result::Error),
    #[error("Can not find latest applied migration number: {0}")]
    CanNotFindLatestAppliedMigrationNumber(InvalidMigrationError),
}

pub fn migrate<T>(
    conn: &mut ft_sdk::Connection,
    migration_sqls: include_dir::Dir,
    migration_functions: std::collections::HashMap<String, T>,
) -> Result<(), MigrationError>
where
    T: FnOnce(&mut ft_sdk::Connection) -> Result<(), diesel::result::Error>,
{
    // check if the migration table exists, if not create it
    create_migration_table(conn).map_err(MigrationError::CanNotCreateMigrationTable)?;

    // find the latest applied migration number from the table
    let latest_applied_migration_number = find_latest_applied_migration_number(conn);

    // find the latest migration number from the migration files
    let latest_migration_number = find_latest_migration_number(migration_sqls, migration_functions)
        .map_err(MigrationError::CanNotFindLatestAppliedMigrationNumber)?;

    // if the latest applied migration number is less than the latest migration number, apply
    // the migrations
    todo!()
}

table! {
    fastn_migration {
        id -> Integer,
        migration_number -> Integer,
        applied_on -> Timestamptz,
    }
}

fn create_migration_table(conn: &mut ft_sdk::Connection) -> Result<(), diesel::result::Error> {
    diesel::dsl::sql_query(
        r#"
        CREATE TABLE IF NOT EXISTS
            fastn_migration
        (
            id INTEGER PRIMARY KEY,
            migration_number INTEGER NOT NULL,
            applied_on INTEGER NOT NULL
        )
    "#,
    )
    .execute(conn)
    .map(|_| ())
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
    InvalidSqlFileNotUtf8(std::ffi::OsString),
}

fn find_latest_migration_number<T>(
    migration_sqls: include_dir::Dir,
    migration_functions: std::collections::HashMap<String, T>,
) -> Result<i32, InvalidMigrationError>
where
    T: FnOnce(&mut ft_sdk::Connection) -> Result<(), diesel::result::Error>,
{
    let mut latest_migration_number = 0;

    for file in migration_sqls.files() {
        let file = file.path().file_name().unwrap();
        let file_name = match file.to_str() {
            Some(v) => v,
            None => return Err(InvalidMigrationError::InvalidSqlFileNotUtf8(file.into())),
        };

        let migration_number = file_name.split('_').next().unwrap().parse().unwrap();
        if migration_number > latest_migration_number {
            latest_migration_number = migration_number;
        }
    }

    for (file_name, _) in migration_functions {
        let migration_number = file_name.split('_').next().unwrap().parse().unwrap();
        if migration_number > latest_migration_number {
            latest_migration_number = migration_number;
        }
    }

    Ok(latest_migration_number)
}
