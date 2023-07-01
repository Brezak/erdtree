use crate::{context::error::Error as CtxError, styles::error::Error as StyleError};
use ignore::Error as IgnoreError;
use std::io::Error as IoError;

#[cfg(unix)]
use crate::fs::permissions::error::Error as PermissionsError;

/// Errors that may occur while traversing or construction of [`Tree`].
///
/// [`Tree`]: super::Tree
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Context(#[from] CtxError),

    #[error("{0}")]
    DirNotFound(String),

    #[error("File expected to have parent")]
    ExpectedParent,

    #[error("Invalid glob patterns: {0}")]
    InvalidGlobPatterns(#[from] IgnoreError),

    #[error("Failed to compute root node.")]
    MissingRoot,

    #[error("No entries to show with given arguments.")]
    NoMatches,

    #[error("{0}")]
    PathCanonicalization(#[from] IoError),

    #[cfg(unix)]
    #[error("{0}")]
    Permissions(#[from] PermissionsError),

    #[error("{0}")]
    UninitializedTheme(#[from] StyleError<'static>),

    #[error("Terminated erdtree...")]
    Terminated,
}
