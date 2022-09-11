use crate::{BinaryOperator, Span, UnaryOperator};
#[derive(Clone, Debug)]
pub struct AtomicExpression {
    pub ident: String,
}

#[derive(Clone, Debug)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Expression,
    pub operator_span: Span,
}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
    pub lhs: Expression,
    pub operator: BinaryOperator,
    pub rhs: Expression,
    pub operator_span: Span,
}

#[derive(Clone, Debug)]
pub struct ParenExpression {
    pub expression: Expression,
}

#[derive(Clone, Debug)]
pub enum ExpressionKind {
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
