use crate::Span;

/// A spanned error containing multiple messages and spans.
///
/// # Example
/// ```rust
/// # use dare::{Error, Span};
/// // create an error
/// let err = Error::new()
///     .with_msg("This is an example message")
///     .with_span(Span::new(2, 5));
///
/// assert_eq!(err.messages()[0], "This is an example message");
/// assert_eq!(err.spans()[0], Span::new(2, 5));
/// ```
#[derive(Clone, Debug)]
pub struct Error {
    messages: Vec<String>,
    spans: Vec<Span>,
}

impl Error {
    /// Creates a new empty [`Error`].
    ///
    /// See [`Self::with_msg`] and [`Self::with_span`].
    pub const fn new() -> Self {
        Self {
            messages: Vec::new(),
            spans: Vec::new(),
        }
    }

    /// Adds a message to `self`.
    pub fn with_msg(mut self, message: impl Into<String>) -> Self {
        self.messages.push(message.into());
        self
    }

    /// Adds a span to `self`.
    pub fn with_span(mut self, span: Span) -> Self {
        self.spans.push(span);
        self
    }

    /// Returns all messages in `self`.
    pub fn messages(&self) -> &[String] {
        &self.messages
    }

    /// Returns all spans in `self`.
    pub fn spans(&self) -> &[Span] {
        &self.spans
    }
}
