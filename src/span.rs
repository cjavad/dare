use std::ops::Range;

/// Describes a part of a string.
///
/// Contains a [`start`](Span::start) and a [`length`](Span::length).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    start: usize,
    length: usize,
}

impl Span {
    /// Creates a new [`Span`].
    pub const fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }

    /// Start of the [`Span`].
    pub const fn start(&self) -> usize {
        self.start
    }

    /// Length of the [`Span`].
    pub const fn length(&self) -> usize {
        self.length
    }

    /// Returns the end.
    ///
    /// This is same as ``self.start() + self.length()``.
    pub const fn end(&self) -> usize {
        self.start + self.length
    }

    /// Returns the range.
    ///
    /// # Example
    /// ```rust
    /// # use dare::Span;
    /// let some_string = "Hello, world!";
    /// let span = Span::new(7, 5);
    ///
    /// assert_eq!(&some_string[span.range()], "world");
    /// ```
    pub const fn range(&self) -> Range<usize> {
        self.start..self.end()
    }
}
