pub struct DbError {
    message: String,
    details: Option<String>,
    hint: Option<String>,
    table_name: Option<String>,
    column_name: Option<String>,
    constraint_name: Option<String>,
    statement_position: Option<i32>,
}

pub fn db_error_to_diesel_error(e: ft_sys_shared::DbError) -> diesel::result::Error {
    use diesel::result::DatabaseErrorKind::*;

    match e {
        ft_sys_shared::DbError::UnableToSendCommand(e) => diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e),
        ),
        ft_sys_shared::DbError::DatabaseError {
            code,
            message,
            details,
            hint,
            table_name,
            column_name,
            constraint_name,
            statement_position,
        } => diesel::result::Error::DatabaseError(
            match code.as_str() {
                "23505" => UniqueViolation,
                "23503" => ForeignKeyViolation,
                "40001" => SerializationFailure,
                "25006" => ReadOnlyTransaction,
                "23502" => NotNullViolation,
                "23514" => CheckViolation,
                _ => Unknown,
            },
            Box::new(DbError {
                message,
                details,
                hint,
                table_name,
                column_name,
                constraint_name,
                statement_position,
            }),
        ),
    }
}

impl diesel::result::DatabaseErrorInformation for DbError {
    fn message(&self) -> &str {
        self.message.as_str()
    }

    fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }

    fn hint(&self) -> Option<&str> {
        self.hint.as_deref()
    }

    fn table_name(&self) -> Option<&str> {
        self.table_name.as_deref()
    }

    fn column_name(&self) -> Option<&str> {
        self.column_name.as_deref()
    }

    fn constraint_name(&self) -> Option<&str> {
        self.constraint_name.as_deref()
    }

    fn statement_position(&self) -> Option<i32> {
        self.statement_position
    }
}
