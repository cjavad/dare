use std::ops::{Add, AddAssign, Range};

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

/// When two [`Span`]s are added the result is a [`Span`] that contains both [`Span`]s.
///
/// # Example
/// ```rust
/// # use dare::Span;
/// assert_eq!(Span::new(0, 2) + Span::new(4, 3), Span::new(0, 7));
/// ```
impl Add for Span {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let start = self.start().min(rhs.start());
        let end = self.end().max(rhs.end());

        Self::new(start, end - start)
    }
}

impl<T> AddAssign<T> for Span
where
    Self: Add<T, Output = Self>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}
