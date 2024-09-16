use core::fmt::{Debug, Display, Formatter};

/// compatibility wrapper until core::error is used everywhere
pub struct PostcardError(postcard::Error);

impl PostcardError {
    pub(crate) fn new(error: postcard::Error) -> Self {
        Self(error)
    }
}

impl Debug for PostcardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for PostcardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl snafu::Error for PostcardError {}
