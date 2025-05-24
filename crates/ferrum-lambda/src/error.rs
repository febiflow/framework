//! Defines the custom error types used throughout this application/library.
//!
//! This module provides a unified error structure (`Error`) and categorizes
//! errors using the `ErrorKind` enum. It also includes conversions from
//! common external error types (like `sqlx::Error`) for easier error handling.

use std::fmt;

use serde::Serialize;

/// Represents the specific category or origin of an [`Error`].
///
/// This enum helps classify errors, allowing for more structured error handling
/// or reporting based on the error's nature. As the application grows,
/// more variants can be added here (e.g., `IO`, `Parsing`, `Network`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ErrorKind {
    /// Indicates an error originating from database operations, likely via `sqlx`.
    Database,
}

/// The primary error type for this application/library.
///
/// It encapsulates both a categorized [`ErrorKind`] and a detailed,
/// human-readable error `message`. This provides context about *what*
/// kind of error occurred and *specifically* what went wrong.
#[derive(Debug, Serialize)]
pub struct Error {
    /// The category or type of the error, based on [`ErrorKind`].
    pub kind: ErrorKind,

    /// A detailed message describing the specific error encountered.
    /// This often includes context or the underlying error message from a library.
    pub message: String,
}

// --- Trait Implementations ---

/// Implements the `Display` trait for the [`Error`] type.
///
/// This allows the error to be easily converted to a user-friendly string,
/// suitable for displaying in logs or to an end-user. It primarily formats
/// the detailed `message` field.
///
/// # Example
///
/// ```rust,ignore
/// // Assuming `err` is an instance of `Error`
/// println!("An error occurred: {}", err);
/// let error_string = err.to_string();
/// ```
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write the human-readable message directly.
        // The `kind` can be checked programmatically if needed.
        write!(f, "{}", self.message)
    }
}

/// Enables infallible conversion from an [`sqlx::Error`] into our custom [`Error`].
///
/// This is crucial for integrating with database operations using the `sqlx` crate.
/// It simplifies error handling by allowing the use of the `?` operator on functions
/// returning `Result<_, sqlx::Error>` within functions that return `Result<_, Error>`.
///
/// When an `sqlx::Error` is converted:
/// - The `kind` is set to [`ErrorKind::Database`].
/// - The `message` includes a prefix and the original error's display message.
///
/// # Example
///
/// ```rust,ignore
/// use crate::{Error, ErrorKind};
/// use sqlx;
///
/// async fn fetch_data() -> Result<(), Error> {
///     let pool = /* ... get sqlx::PgPool ... */;
///     // The `?` operator automatically calls `Error::from(sqlx::Error)` if query fails
///     let _result = sqlx::query!("SELECT 1").fetch_one(&pool).await?;
///     Ok(())
/// }
/// ```
impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self {
            kind: ErrorKind::Database,
            message: format!("Database Error: {}", error),
        }
    }
}
