//! Building blocks of the encrypted synchronization of local data through a dumb object store.
//!
//! The pieces, from the bottom up:
//!
//! - [`crypto`]: authenticated encryption, so a backend only ever stores ciphertext;
//! - [`keys`]: where the encryption key lives (system keyring, a `0600` file or an environment variable);
//! - [`protected`]: the passphrase-sealed key file;
//! - [`backend`]: the transport interface and a directory backend; [`s3`] and [`sigv4`] add an
//!   S3-compatible one, configured through [`credentials`] and [`setup`];
//! - [`lock`]: the single-instance lock of a synchronization run.
//!
//! What is synchronized (the document and its merge) belongs to the application. Sessions,
//! passwords, cloud credentials and the key itself must never be part of it.

use thiserror::Error;

pub mod backend;
pub mod credentials;
pub mod crypto;
/// An in-memory S3 for tests, shared with the applications through the `test-support` feature.
#[cfg(any(test, feature = "test-support"))]
pub mod fake_s3;
pub mod keys;
pub mod lock;
pub mod protected;
pub mod s3;
pub mod setup;
pub mod sigv4;

pub use backend::{Condition, DirectoryBackend, Object, PutOutcome, SyncBackend};
pub use credentials::S3Credentials;
pub use crypto::{decrypt, encrypt, Key};
pub use keys::{key_exists, load_key, store_key, KeySource, Passphrase, KEY_ENV, PASSPHRASE_ENV};
pub use lock::SyncLock;
pub use s3::{S3Backend, S3Settings};
pub use setup::{
    backend_from, key_source_from, validate_settings, AnyBackend, DIRECTORY_BACKEND, R2_BACKEND,
    S3_BACKEND,
};

/// Everything that can go wrong while synchronizing.
#[derive(Debug, Error)]
pub enum SyncError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid synced document: {0}")]
    Json(#[from] serde_json::Error),
    #[error("encryption key: {0}")]
    Key(String),
    #[error("{0}")]
    Crypto(String),
    #[error("keyring: {0}")]
    Keyring(#[from] keyring_core::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("backend: {0}")]
    Backend(String),
    #[error("could not agree with the remote copy after several attempts")]
    Conflict,
    #[error(transparent)]
    Config(#[from] suap_core::SuapError),
}
