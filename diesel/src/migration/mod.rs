//! Representation of migrations

extern crate serde;
use self::serde::de::DeserializeOwned;

mod errors;
pub use self::errors::{MigrationError, RunMigrationsError};

use connection::SimpleConnection;
use std::path::Path;

/// Represents a migration that interacts with diesel
pub trait Migration<M>
where
    M : Metadata + Sized
{
    /// Get the migration version
    fn version(&self) -> &str;

    /// Apply this migration
    fn run(&self, conn: &SimpleConnection) -> Result<(), RunMigrationsError>;

    /// Revert this migration
    fn revert(&self, conn: &SimpleConnection) -> Result<(), RunMigrationsError>;

    /// Get the migration file path
    fn file_path(&self) -> Option<&Path> {
        None
    }

    /// Get the metadata associated with this migration, if any
    fn metadata(&self) -> Option<&M> {
        None
    }
}

impl<M> Migration<M> for Box<Migration<M>>
where
    M : Metadata + Sized
{
    fn version(&self) -> &str {
        (&**self).version()
    }

    fn run(&self, conn: &SimpleConnection) -> Result<(), RunMigrationsError> {
        (&**self).run(conn)
    }

    fn revert(&self, conn: &SimpleConnection) -> Result<(), RunMigrationsError> {
        (&**self).revert(conn)
    }

    fn file_path(&self) -> Option<&Path> {
        (&**self).file_path()
    }

    fn metadata(&self) -> Option<&M> {
        (&**self).metadata()
    }
}

impl<'a, M> Migration<M> for &'a Migration<M>
where
    M : Metadata + Sized
{
    fn version(&self) -> &str {
        (&**self).version()
    }

    fn run(&self, conn: &SimpleConnection) -> Result<(), RunMigrationsError> {
        (&**self).run(conn)
    }

    fn revert(&self, conn: &SimpleConnection) -> Result<(), RunMigrationsError> {
        (&**self).revert(conn)
    }

    fn file_path(&self) -> Option<&Path> {
        (&**self).file_path()
    }

    fn metadata(&self) -> Option<&M> {
        (&**self).metadata()
    }
}

/// Represents metadata associated with a migration.
///
/// The format of a migration's metadata is dependent on the migration format
/// being used.
///
/// For Diesel's built in SQL file migrations, metadata is stored in a file
/// called `metadata.toml`. Diesel looks for a single key, `run_in_transaction`.
/// By default, all migrations are run in a transaction on SQLite and
/// PostgreSQL. This behavior can be disabled for a single migration by setting
/// this to `false`.
pub trait Metadata {
    /// Get the metadata at the given key, if present
    fn get<T>(&self, key: &str) -> Option<Result<T, MigrationError>>
    where
        Self : Sized,
        T : DeserializeOwned;
}

/// This struct represents non-existent metadata for a migration. It can be used for Migrations
/// that wish to never return anything other than None in their `metadata()` method.
#[derive(Copy, Clone, Debug)]
pub struct DummyMetadata {
}

impl Metadata for DummyMetadata {
    fn get<T>(&self, _key: &str) -> Option<Result<T, MigrationError>>
    where
        T : DeserializeOwned
    {
        None
    }
}
