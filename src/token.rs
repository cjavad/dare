use crate::Span;

/// A token in a logical expression.
///
/// Contains a [`TokenKind`] and a [`Span`].
#[derive(Clone, Debug)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    /// Creates a new [`Token`].
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Returns the [`TokenKind`].
    pub const fn kind(&self) -> &TokenKind {
        &self.kind
    }

    /// Returns the [`Span`].
    pub const fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Identifier(String),
    Delimiter(Delimiter),
    Operation(Operation),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Delimiter {
    Open,
    Close,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Operation {
    Negation,
    Conjunction,
    Disjunction,
    Implication,
    BiImplication,
}

impl Operation {
    /// Returns the precedence.
    ///
    /// **Note** that a lower number means a higher precedence.
    pub const fn precedence(&self) -> i32 {
        match self {
            Operation::Negation => 0,
            Operation::Conjunction => 1,
            Operation::Disjunction => 2,
            Operation::Implication => 3,
            Operation::BiImplication => 4,
        }
    }
}
