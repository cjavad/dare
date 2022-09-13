use crate::{
    AtomicExpression, BinaryOperator, Error, Expression, ExpressionKind, TruthValueExpression,
    UnaryOperator,
};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Interpreter {
    pub variables: HashMap<String, bool>,
    pub atomic_expressions: Vec<AtomicExpression>,
    pub errors: Vec<Error>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extract_atomic_expressions(&mut self, expression: &Expression) {
        match expression.kind.as_ref() {
            ExpressionKind::Atomic(atomic_expression) => {
                self.atomic_expressions.push(atomic_expression.clone())
            }
            ExpressionKind::Unary(unary_expression) => {
                self.extract_atomic_expressions(&unary_expression.operand)
            }
            ExpressionKind::Binary(binary_expression) => {
                self.extract_atomic_expressions(&binary_expression.lhs);
                self.extract_atomic_expressions(&binary_expression.rhs);
            }
            ExpressionKind::TruthValue(_) => {}
            ExpressionKind::Paren(paren_expression) => {
                self.extract_atomic_expressions(&paren_expression.expression)
            }
        }
    }

    pub fn interpret(&mut self, expression: &Expression) -> Result<TruthValueExpression, Error> {
        match expression.kind.as_ref() {
            ExpressionKind::Paren(expression) => self.interpret(&expression.expression),
            ExpressionKind::Atomic(expression) => {
                self.atomic_expressions.push(expression.clone());

                if let Some(value) = self.variables.get(&expression.ident) {
                    Ok(TruthValueExpression {
                        value: value.clone(),
                        span: expression.span.clone(),
                    })
                } else {
                    let error = Error::new()
                        .with_msg("unknown identifier")
                        .with_span(expression.span.clone());
                    self.errors.push(error.clone());
                    Err(error)
                }
            }
            ExpressionKind::Unary(expression) => {
                let operand = self.interpret(&expression.operand)?;
                match expression.operator {
                    UnaryOperator::Negation(_) => Ok(TruthValueExpression {
                        value: !operand.value,
                        span: expression.operator_span.clone(),
                    }),
                }
            }
            ExpressionKind::Binary(expression) => {
                let rhs = self.interpret(&expression.rhs)?;

                match expression.operator {
                    // FIXME(javad): PLEASE FIX
                    /*
                    BinaryOperator::Assignment(_) => {
                        if let ExpressionKind::Atomic(atomic) = expression.lhs.kind.as_ref() {
                            self.variables.insert(atomic.ident.clone(), rhs.value);
                            Ok(TruthValueExpression {
                                value: rhs.value,
                                span: expression.operator_span.clone(),
                            })
                        } else {
                            let error = Error::new()
                                .with_msg("invalid assignment")
                                .with_span(expression.operator_span.clone());
                            self.errors.push(error.clone());
                            Err(error)
                        }
                    }
                    */
                    operator => {
                        let lhs = self.interpret(&expression.lhs)?;

                        match operator {
                            BinaryOperator::Conjunction(_) => Ok(TruthValueExpression {
                                value: lhs.value && rhs.value,
                                span: expression.operator_span.clone(),
                            }),
                            BinaryOperator::Disjunction(_) => Ok(TruthValueExpression {
                                value: lhs.value || rhs.value,
                                span: expression.operator_span.clone(),
                            }),
                            BinaryOperator::ExclusiveDisjunction(_) => Ok(TruthValueExpression {
                                value: lhs.value ^ rhs.value,
                                span: expression.operator_span.clone(),
                            }),

                            BinaryOperator::Implication(_) => Ok(TruthValueExpression {
                                value: !lhs.value || rhs.value,
                                span: expression.operator_span.clone(),
                            }),
                            BinaryOperator::Equivalence(_) => Ok(TruthValueExpression {
                                value: lhs.value == rhs.value,
                                span: expression.operator_span.clone(),
                            }),
                        }
                    }
                }
            }
            ExpressionKind::TruthValue(truthvalue_expr) => Ok(TruthValueExpression {
                value: truthvalue_expr.value,
                span: truthvalue_expr.span.clone(),
            }),
        }
    }
}

// Test

// Language: rust
// Path: src\interpreter.rs
// Compare this snippet from src\parser.rs:
#[cfg(test)]
mod tests {
    use crate::Parser;

    use super::*;

    #[test]
    fn test_interpret() {
        macro_rules! interpreter_test_identical {
            ($source:expr, $expected:expr, $identical_bool:expr) => {
                let parser = Parser::new();
                let expr = parser.parse($source).unwrap();
                let mut interpreter = Interpreter::new();
                interpreter.extract_atomic_expressions(&expr);
                for atomic_expr in &interpreter.atomic_expressions {
                    interpreter
                        .variables
                        .insert(atomic_expr.ident.clone(), $identical_bool);
                }
                let result = interpreter.interpret(&expr).unwrap();
                assert_eq!(result.value, $expected);
            };
        }

        interpreter_test_identical!("a", true, true);
        interpreter_test_identical!("a", false, false);
        interpreter_test_identical!("!a", false, true);
        interpreter_test_identical!("!a", true, false);
        interpreter_test_identical!("a && b", true, true);
        interpreter_test_identical!("a && b", false, false);
        interpreter_test_identical!("a || b", true, true);
        interpreter_test_identical!("a || b", false, false);
        interpreter_test_identical!("a ^ b", false, false);
        interpreter_test_identical!("a ^ b", false, true);
        interpreter_test_identical!("a -> b", true, true);
        interpreter_test_identical!("a -> b", true, false);
        interpreter_test_identical!("a -> b", true, true);
        interpreter_test_identical!("a -> b", true, false);
        interpreter_test_identical!("a <-> b", true, true);
        interpreter_test_identical!("a <-> b", true, false);
        interpreter_test_identical!("!(a) -> !b", true, true);
        // De Morgan's laws
        interpreter_test_identical!("!(a & b) == (!a & !b)", true, true);
        interpreter_test_identical!("!(a | b) == (!a | !b)", true, true);
        // Associativity
        interpreter_test_identical!("a & b & c", true, true);
        interpreter_test_identical!("a | b | c", true, true);
        interpreter_test_identical!("a ^ b ^ c", true, true);
        // Commutativity
        interpreter_test_identical!("a & b == b & a", true, true);
        interpreter_test_identical!("a | b == b | a", true, true);
        interpreter_test_identical!("a ^ b == b ^ a", true, true);
        // Distributivity
        interpreter_test_identical!("a & (b | c) == (a & b) | (a & c)", true, true);
        interpreter_test_identical!("a | (b & c) == (a | b) & (a | c)", true, true);
        // Idempotence
        interpreter_test_identical!("a & a == a", true, true);
        interpreter_test_identical!("a | a == a", true, true);
        interpreter_test_identical!("a ^ a == a", true, false);
        // Absorption
        interpreter_test_identical!("a & (a | b) == a", true, true);
        interpreter_test_identical!("a | (a & b) == a", true, true);
        // Identity
        interpreter_test_identical!("a & 1 == a", true, true);
        interpreter_test_identical!("a | 0 == a", true, true);
        // Negation
        interpreter_test_identical!("a & !a == 0", true, true);
        interpreter_test_identical!("a | !a == 1", true, true);
        // Double negation
        interpreter_test_identical!("!!a == a", true, true);
        // Absorption
        interpreter_test_identical!("a & (a | b) == a", true, true);
        interpreter_test_identical!("a | (a & b) == a", true, true);
    }
}
