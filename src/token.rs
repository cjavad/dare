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
    TruthValue(bool),
    Identifier(String),
    Delimiter(Delimiter),
    Assignment(&'static str),
    UnaryOperator(UnaryOperator),
    BinaryOperator(BinaryOperator),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Delimiter {
    Open,
    Close,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Negation(&'static str),
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self::Negation(symbol) = self;
        f.write_str(symbol)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Conjunction(&'static str),
    Disjunction(&'static str),
    ExclusiveDisjunction(&'static str),
    Implication(&'static str),
    Equivalence(&'static str),
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Conjunction(symbol) => f.write_str(symbol),
            BinaryOperator::Disjunction(symbol) => f.write_str(symbol),
            BinaryOperator::ExclusiveDisjunction(symbol) => f.write_str(symbol),
            BinaryOperator::Implication(symbol) => f.write_str(symbol),
            BinaryOperator::Equivalence(symbol) => f.write_str(symbol),
        }
    }
}

impl BinaryOperator {
    /// Returns the precedence.
    ///
    /// **Note** that a lower number means a higher precedence.
    pub const fn precedence(&self) -> i32 {
        match self {
            BinaryOperator::Conjunction(_) => 1,
            BinaryOperator::Disjunction(_) => 2,
            BinaryOperator::ExclusiveDisjunction(_) => 3,
            BinaryOperator::Implication(_) => 4,
            BinaryOperator::Equivalence(_) => 5,
        }
    }

    pub const fn is_associative(&self) -> bool {
        match self {
            BinaryOperator::Conjunction(_) => true,
            BinaryOperator::Disjunction(_) => true,
            BinaryOperator::ExclusiveDisjunction(_) => true,
            BinaryOperator::Implication(_) => false,
            BinaryOperator::Equivalence(_) => false,
        }
    }
}
