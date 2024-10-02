use core::fmt::{Debug, Display, Formatter};

/// compatibility wrapper until core::error is used everywhere
#[repr(transparent)]
pub struct PostcardError(postcard::Error);

impl PostcardError {
    pub(crate) fn new(error: postcard::Error) -> Self {
        Self(error)
    }
}

// Debug trait is used on errors to generate developer targeted information. Required by snafu::Error
impl Debug for PostcardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

// Display trait is used on errors to generate the error message itself. Required by snafu::Error
impl Display for PostcardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

// mark PostcardError as compatible to snafu::Error trait
impl snafu::Error for PostcardError {}
