use diesel::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
    #[error("Can not create migration table: {0}")]
    CanNotCreateMigrationTable(diesel::result::Error),
    #[error("Can not find latest applied migration number: {0}")]
    CanNotFindLatestAppliedMigrationNumber(diesel::result::Error),
    #[error("Invalid migration: {0}")]
    InvalidMigration(#[from] InvalidMigrationError),
    #[error("Invalid migration numbers: {0} < {1}, migration deleted?")]
    InvalidMigrationNumbers(i32, i32),
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

    // find the latest migration number from the migration files
    let latest_migration_number =
        find_latest_migration_number(&migration_sqls, &migration_functions)?;

    #[allow(clippy::comparison_chain)]
    if latest_migration_number == latest_applied_migration_number {
        ft_sdk::println!("No new migrations to apply");
    } else if latest_migration_number > latest_applied_migration_number {
        apply_migrations(
            conn,
            &migration_sqls,
            &migration_functions,
            latest_applied_migration_number,
        )?;
    } else {
        return Err(MigrationError::InvalidMigrationNumbers(
            latest_migration_number.unwrap(),
            latest_applied_migration_number.unwrap(),
        ));
    }

    Ok(())
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
    #[error("SQL file is not integer: {0:?}")]
    SqlFileIsNotInteger(#[from] std::num::ParseIntError),
}

fn find_latest_migration_number<T>(
    migration_sqls: &include_dir::Dir,
    migration_functions: &std::collections::HashMap<i32, T>,
) -> Result<Option<i32>, InvalidMigrationError>
where
    T: FnOnce(&mut ft_sdk::Connection) -> Result<(), diesel::result::Error>,
{
    let mut latest_migration_number = None;

    for file in migration_sqls.files() {
        if file.path().extension() != Some(std::ffi::OsStr::new("sql")) {
            continue;
        }

        let file_stem = file.path().file_stem().unwrap();
        let file_stem = match file_stem.to_str() {
            Some(v) => v,
            None => {
                return Err(InvalidMigrationError::InvalidSqlFileNotUtf8(
                    file_stem.into(),
                ))
            }
        };

        let migration_number = file_stem.parse()?;

        if migration_functions.contains_key(&migration_number) {
            return Err(InvalidMigrationError::DuplicateMigration(migration_number));
        }

        let migration_number = Some(migration_number);

        if migration_number > latest_migration_number {
            latest_migration_number = migration_number;
        }
    }

    for migration_number in migration_functions.keys() {
        let migration_number = Some(*migration_number);

        if migration_number > latest_migration_number {
            latest_migration_number = migration_number;
        }
    }

    Ok(latest_migration_number)
}

pub fn apply_migrations<T>(
    _conn: &mut ft_sdk::Connection,
    _migration_sqls: &include_dir::Dir,
    _migration_functions: &std::collections::HashMap<i32, T>,
    _latest_applied_migration_number: Option<i32>,
) -> Result<(), MigrationError>
where
    T: FnOnce(&mut ft_sdk::Connection) -> Result<(), diesel::result::Error>,
{
    todo!()
}
