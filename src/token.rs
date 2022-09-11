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
    UnaryOperator(UnaryOperator),
    BinaryOperator(BinaryOperator),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Delimiter {
    Open,
    Close,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Conjunction,
    Disjunction,
    Implication,
    Equivalence,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Negation,
}

impl BinaryOperator {
    /// Returns the precedence.
    ///
    /// **Note** that a lower number means a higher precedence.
    pub const fn precedence(&self) -> i32 {
        match self {
            BinaryOperator::Conjunction => 1,
            BinaryOperator::Disjunction => 2,
            BinaryOperator::Implication => 3,
            BinaryOperator::Equivalence => 4,
        }
    }

    pub const fn is_associative(&self) -> bool {
        match self {
            BinaryOperator::Conjunction => true,
            BinaryOperator::Disjunction => true,
            BinaryOperator::Implication => false,
            BinaryOperator::Equivalence => false,
        }
    }
}
