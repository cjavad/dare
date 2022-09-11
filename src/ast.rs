use crate::Span;

pub enum UnaryOperatorKind {
    Negation
}

pub struct UnaryOperator {
    pub kind: UnaryOperatorKind,
    pub span: Span,
}

pub enum BinaryOperatorKind {
    Conjunction,
    Disjunction,
    Implication,
    Equivalence,
}

pub struct BinaryOperator {
    pub kind: BinaryOperatorKind,
    pub span: Span,
}

pub struct AtomicExpression {
    pub ident: String,
}

pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Expression
}

pub struct BinaryExpression {
    pub lhs: Expression,
    pub operator: BinaryOperator,
    pub rhs: Expression,
}

pub struct ParenExpression {
    pub expression: Expression,
}

pub enum ExpressionKind {
    Atomic(AtomicExpression),
    Paren(ParenExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
}

pub struct Expression {
    pub kind: Box<ExpressionKind>,
    pub span: Span,
}