// need a type that can implement traits for storing custom data

use std::{borrow::Cow, error, fmt};

/// Data that can be loaded from the clipboard.
pub struct ClipboardLoadData<T>(pub T);

/// Describes the mime types which are accepted.
pub trait AllowedMimeTypes:
    TryFrom<(Vec<u8>, String)> + Send + Sync + 'static
{
    /// List allowed mime types for the type to convert from a byte slice.
    ///
    /// Allowed mime types should be listed in order of decreasing preference,
    /// most preferred first.
    fn allowed() -> Cow<'static, [String]>;
}

/// Can be converted to data with the available mime types.
pub trait AsMimeTypes {
    /// List available mime types for this data to convert to a byte slice.
    fn available(&self) -> Cow<'static, [String]>;

    /// Converts a type to a byte slice for the given mime type if possible.
    fn as_bytes(&self, mime_type: &str) -> Option<Cow<'static, [u8]>>;
}

/// Data that can be stored to the clipboard.
pub struct ClipboardStoreData<T> {
    /// Clipboard data.
    pub data: T,
    /// Available mime types for the clipboard data.
    pub available_mime_types: Vec<Cow<'static, str>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unsupported mime type")
    }
}

impl error::Error for Error {}
