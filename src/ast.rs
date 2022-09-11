use crate::Span;

#[derive(Clone, Debug)]
pub enum UnaryOperatorKind {
    Negation,
}

#[derive(Clone, Debug)]
pub struct UnaryOperator {
    pub kind: UnaryOperatorKind,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum BinaryOperatorKind {
    Conjunction,
    Disjunction,
    Implication,
    Equivalence,
}

#[derive(Clone, Debug)]
pub struct BinaryOperator {
    pub kind: BinaryOperatorKind,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct AtomicExpression {
    pub ident: String,
}

#[derive(Clone, Debug)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Expression,
}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
    pub lhs: Expression,
    pub operator: BinaryOperator,
    pub rhs: Expression,
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
