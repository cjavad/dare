use crate::{BinaryOperator, Span, UnaryOperator};

#[derive(Clone, Debug)]
pub struct TruthValueExpression {
    pub value: bool,
}

impl std::fmt::Display for TruthValueExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.value {
                true => "1",
                false => "0",
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct AtomicExpression {
    pub ident: String,
}

impl std::fmt::Display for AtomicExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident)
    }
}

#[derive(Clone, Debug)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Expression,
    pub operator_span: Span,
}

impl std::fmt::Display for UnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.operator, self.operand)
    }
}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
    pub lhs: Expression,
    pub operator: BinaryOperator,
    pub rhs: Expression,
    pub operator_span: Span,
}

impl std::fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.operator, self.rhs)
    }
}

#[derive(Clone, Debug)]
pub struct ParenExpression {
    pub expression: Expression,
}

impl std::fmt::Display for ParenExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.expression)
    }
}

#[derive(Clone, Debug)]
pub enum ExpressionKind {
    TruthValue(TruthValueExpression),
    Atomic(AtomicExpression),
    Paren(ParenExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
}

#[derive(Clone, Debug)]
pub struct Expression {
    pub kind: Box<ExpressionKind>,
    pub span: Span,
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind.as_ref() {
            ExpressionKind::TruthValue(expr) => write!(f, "{}", expr),
            ExpressionKind::Atomic(expr) => expr.fmt(f),
            ExpressionKind::Paren(expr) => expr.fmt(f),
            ExpressionKind::Unary(expr) => expr.fmt(f),
            ExpressionKind::Binary(expr) => expr.fmt(f),
        }
    }
}
