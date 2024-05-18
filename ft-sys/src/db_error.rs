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
            kind,
            message,
            details,
            hint,
            table_name,
            column_name,
            constraint_name,
            statement_position,
            ..
        } => diesel::result::Error::DatabaseError(
            match kind {
                ft_sys_shared::DatabaseErrorKind::UniqueViolation => UniqueViolation,
                ft_sys_shared::DatabaseErrorKind::ForeignKeyViolation => ForeignKeyViolation,
                ft_sys_shared::DatabaseErrorKind::NotNullViolation => NotNullViolation,
                ft_sys_shared::DatabaseErrorKind::CheckViolation => CheckViolation,
                ft_sys_shared::DatabaseErrorKind::SerializationFailure => SerializationFailure,
                ft_sys_shared::DatabaseErrorKind::ReadOnlyTransaction => ReadOnlyTransaction,
                ft_sys_shared::DatabaseErrorKind::ClosedConnection => ClosedConnection,
                ft_sys_shared::DatabaseErrorKind::Unknown => Unknown,
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
