use crate::{
    AtomicExpression, BinaryExpression, Delimiter, Error, Expression, ExpressionKind,
    ParenExpression, TokenKind, TokenStream, TruthValueExpression, UnaryExpression,
};

#[derive(Clone, Debug, Default)]
pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse_atomic_expr(&self, tokens: &mut TokenStream) -> Result<AtomicExpression, Error> {
        let next_token = tokens.next()?;
        match next_token.kind() {
            TokenKind::Identifier(identifier) => Ok(AtomicExpression {
                ident: identifier.clone(),
                span: next_token.span().clone(),
            }),
            _ => Err(Error::new().with_msg("expected atomic expression")),
        }
    }

    pub fn parse_truthvalue_expr(
        &self,
        tokens: &mut TokenStream,
    ) -> Result<TruthValueExpression, Error> {
        let next_token = tokens.next()?;
        match next_token.kind() {
            TokenKind::TruthValue(value) => Ok(TruthValueExpression {
                value: value.clone(),
                span: next_token.span().clone(),
            }),
            _ => Err(Error::new().with_msg("expected truth value expression")),
        }
    }

    pub fn parse_paren_expr(&self, tokens: &mut TokenStream) -> Result<ParenExpression, Error> {
        tokens.expect(&TokenKind::Delimiter(Delimiter::Open))?;
        let expression = self.parse(tokens)?;
        tokens.expect(&TokenKind::Delimiter(Delimiter::Close))?;
        Ok(ParenExpression { expression })
    }

    pub fn parse_term_expr(&self, tokens: &mut TokenStream) -> Result<Expression, Error> {
        match tokens.peek()?.kind() {
            TokenKind::Delimiter(Delimiter::Open) => {
                let start_span = tokens.span();
                let expression = self.parse_paren_expr(tokens)?;
                let end_span = tokens.span();
                Ok(Expression {
                    kind: Box::new(ExpressionKind::Paren(expression)),
                    span: start_span + end_span,
                })
            }
            TokenKind::Identifier(_) => {
                let start_span = tokens.span();
                let expression = self.parse_atomic_expr(tokens)?;
                let end_span = tokens.span();
                Ok(Expression {
                    kind: Box::new(ExpressionKind::Atomic(expression)),
                    span: start_span + end_span,
                })
            }
            TokenKind::TruthValue(_) => {
                let start_span = tokens.span();
                let expression = self.parse_truthvalue_expr(tokens)?;
                let end_span = tokens.span();
                Ok(Expression {
                    kind: Box::new(ExpressionKind::TruthValue(expression)),
                    span: start_span + end_span,
                })
            }
            _ => {
                let error = Error::new()
                    .with_msg("expected expression")
                    .with_span(tokens.span());
                Err(error)
            }
        }
    }

    pub fn parse_unary_expr(&self, tokens: &mut TokenStream) -> Result<Expression, Error> {
        let start_span = tokens.span();

        match tokens.try_peek_kind() {
            Some(TokenKind::UnaryOperator(operator)) => {
                let operator = operator.clone();
                let operator_span = tokens.next()?.span();
                let operand = self.parse_unary_expr(tokens)?;
                let end_span = tokens.span();
                Ok(Expression {
                    kind: Box::new(ExpressionKind::Unary(UnaryExpression {
                        operator,
                        operand,
                        operator_span,
                    })),
                    span: start_span + end_span,
                })
            }
            _ => self.parse_term_expr(tokens),
        }
    }

    pub fn parse(&self, tokens: &mut TokenStream) -> Result<Expression, Error> {
        let start_span = tokens.span();
        let lhs = self.parse_unary_expr(tokens)?;

        match tokens.try_peek_kind() {
            Some(TokenKind::BinaryOperator(operator)) => {
                let operator = operator.clone();
                let operator_span = tokens.next()?.span();
                let rhs = self.parse(tokens)?;
                let end_span = tokens.span();

                if let ExpressionKind::Binary(rhs_expr) = rhs.kind.as_ref() {
                    if operator.precedence() == rhs_expr.operator.precedence() {
                        if !operator.is_associative() || !rhs_expr.operator.is_associative() {
                            let error = Error::new()
                                .with_msg("non-associative operators must be parenthesized")
                                .with_span(start_span + end_span);
                            return Err(error);
                        }
                    }

                    if operator.precedence() < rhs_expr.operator.precedence() {
                        let expression = Expression {
                            kind: Box::new(ExpressionKind::Binary(BinaryExpression {
                                lhs,
                                operator,
                                rhs: rhs_expr.lhs.clone(),
                                operator_span,
                            })),
                            span: rhs.span,
                        };

                        return Ok(Expression {
                            kind: Box::new(ExpressionKind::Binary(BinaryExpression {
                                lhs: expression,
                                operator: rhs_expr.operator.clone(),
                                rhs: rhs_expr.rhs.clone(),
                                operator_span: rhs_expr.operator_span,
                            })),
                            span: start_span + end_span,
                        });
                    }
                }

                Ok(Expression {
                    kind: Box::new(ExpressionKind::Binary(BinaryExpression {
                        lhs,
                        operator,
                        rhs,
                        operator_span,
                    })),
                    span: start_span + end_span,
                })
            }
            _ => Ok(lhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let parser = Parser::new();

        macro_rules! parser_tests {
            ($($source:literal),* $(,)?) => {$({
                let mut tokens = TokenStream::parse($source).unwrap();
                let expr = parser.parse(&mut tokens).unwrap();
                assert_eq!(expr.to_string(), $source);
            })*};
        }

        parser_tests! {
            "A",
            "~A",
            "A & B",
            "A | B",
            "A -> B",
            "A <-> B",
            "A ⊕ B",
            "A ^ B",
            "(A -> B) & (B -> C)",
            "A -> (B -> C)",
            "(A -> B) -> C",
            "(A -> B) & (B -> C) & (C -> D)",
            "(A -> B)",
        }
    }
}
