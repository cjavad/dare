use crate::{
    AtomicExpression, BinaryExpression, BinaryOperator, Expression, ExpressionKind,
    ParenExpression, TruthValueExpression, UnaryExpression, UnaryOperator,
};

#[derive(Clone, Debug)]
pub struct Expectation {
    pub expr: Expression,
    pub truth_value: bool,
}

impl Expectation {
    pub const fn new(expr: Expression, truth_value: bool) -> Self {
        Self { expr, truth_value }
    }
}

#[derive(Clone, Debug)]
pub struct Tableau {
    pub expectations: Vec<Expectation>,
    pub branches: Vec<Tableau>,
}

impl Tableau {
    pub fn merge(&mut self, mut other: Self) {
        self.expectations.append(&mut other.expectations);
        self.branches.append(&mut other.branches);
    }

    pub fn append(&mut self, other: Self) {
        self.expectations.append(&mut other.expectations.clone());

        if self.branches.is_empty() {
            self.branches = other.branches;

            return;
        }

        for branch in self.branches.iter_mut() {
            branch.append(other.clone());
        }
    }

    pub fn width(&self) -> usize {
        if self.branches.is_empty() {
            return 1;
        }

        self.branches
            .iter()
            .map(|branch| branch.width())
            .sum::<usize>()
    }
}

#[derive(Default)]
pub struct TableauBuilder {}

impl TableauBuilder {
    #[must_use]
    pub fn build_truth_value(
        &self,
        expr: &Expression,
        _truth_value: &TruthValueExpression,
        expect: bool,
    ) -> Tableau {
        Tableau {
            expectations: vec![Expectation::new(expr.clone(), expect)],
            branches: Vec::new(),
        }
    }

    #[must_use]
    pub fn build_atomic(
        &self,
        expr: &Expression,
        _atomic: &AtomicExpression,
        expect: bool,
    ) -> Tableau {
        Tableau {
            expectations: vec![Expectation::new(expr.clone(), expect)],
            branches: Vec::new(),
        }
    }

    #[must_use]
    pub fn build_paren(
        &self,
        _expr: &Expression,
        paren: &ParenExpression,
        expect: bool,
    ) -> Tableau {
        self.build_expression(&paren.expression, expect)
    }

    #[must_use]
    pub fn build_negation(&self, operand: &Expression, expect: bool) -> Tableau {
        self.build_expression(&operand, !expect)
    }

    #[must_use]
    pub fn build_unary(&self, expr: &Expression, unary: &UnaryExpression, expect: bool) -> Tableau {
        let tableau = match unary.operator {
            UnaryOperator::Negation(_) => self.build_expression(&unary.operand, !expect),
        };

        Tableau {
            expectations: vec![Expectation::new(expr.clone(), expect)],
            branches: vec![tableau],
        }
    }

    #[must_use]
    pub fn build_conjunction(
        &self,
        expr: &Expression,
        lhs: &Expression,
        rhs: &Expression,
        expect: bool,
    ) -> Tableau {
        if expect == true {
            let mut lhs_tableau = self.build_expression(lhs, true);
            let rhs_tableau = self.build_expression(rhs, true);

            lhs_tableau.append(rhs_tableau);

            Tableau {
                expectations: vec![Expectation::new(expr.clone(), true)],
                branches: vec![lhs_tableau],
            }
        } else {
            let lhs_tableau = self.build_expression(lhs, false);
            let rhs_tableau = self.build_expression(rhs, false);

            Tableau {
                expectations: vec![Expectation::new(expr.clone(), false)],
                branches: vec![lhs_tableau, rhs_tableau],
            }
        }
    }

    #[must_use]
    pub fn build_disjunction(
        &self,
        expr: &Expression,
        lhs: &Expression,
        rhs: &Expression,
        expect: bool,
    ) -> Tableau {
        if expect == true {
            let lhs_tableau = self.build_expression(lhs, true);
            let rhs_tableau = self.build_expression(rhs, true);

            Tableau {
                expectations: vec![Expectation::new(expr.clone(), true)],
                branches: vec![lhs_tableau, rhs_tableau],
            }
        } else {
            let mut lhs_tableau = self.build_expression(lhs, false);
            let rhs_tableau = self.build_expression(rhs, false);

            lhs_tableau.append(rhs_tableau);

            Tableau {
                expectations: vec![Expectation::new(expr.clone(), true)],
                branches: vec![lhs_tableau],
            }
        }
    }

    #[must_use]
    pub fn build_exclusive(
        &self,
        expr: &Expression,
        lhs: &Expression,
        rhs: &Expression,
        expect: bool,
    ) -> Tableau {
        self.build_equivalence(expr, lhs, rhs, !expect)
    }

    #[must_use]
    pub fn build_implication(
        &self,
        expr: &Expression,
        lhs: &Expression,
        rhs: &Expression,
        expect: bool,
    ) -> Tableau {
        if expect == true {
            let lhs_tableau = self.build_expression(lhs, false);
            let rhs_tableau = self.build_expression(rhs, true);

            Tableau {
                expectations: vec![Expectation::new(expr.clone(), true)],
                branches: vec![lhs_tableau, rhs_tableau],
            }
        } else {
            let mut lhs_tableau = self.build_expression(lhs, true);
            let rhs_tableau = self.build_expression(rhs, false);

            lhs_tableau.append(rhs_tableau);

            Tableau {
                expectations: vec![Expectation::new(expr.clone(), false)],
                branches: vec![lhs_tableau],
            }
        }
    }

    #[must_use]
    pub fn build_equivalence(
        &self,
        expr: &Expression,
        lhs: &Expression,
        rhs: &Expression,
        expect: bool,
    ) -> Tableau {
        if expect == true {
            let mut lhs_a = self.build_expression(lhs, true);
            let rhs_a = self.build_expression(rhs, true);

            lhs_a.append(rhs_a);

            let mut lhs_b = self.build_expression(lhs, false);
            let rhs_b = self.build_expression(rhs, false);

            lhs_b.append(rhs_b);

            Tableau {
                expectations: vec![Expectation::new(expr.clone(), true)],
                branches: vec![lhs_a, lhs_b],
            }
        } else {
            let mut lhs_a = self.build_expression(lhs, true);
            let rhs_a = self.build_expression(rhs, false);

            lhs_a.append(rhs_a);

            let mut lhs_b = self.build_expression(lhs, false);
            let rhs_b = self.build_expression(rhs, true);

            lhs_b.append(rhs_b);

            Tableau {
                expectations: vec![Expectation::new(expr.clone(), false)],
                branches: vec![lhs_a, lhs_b],
            }
        }
    }

    #[must_use]
    pub fn build_binary(
        &self,
        expr: &Expression,
        binary: &BinaryExpression,
        expect: bool,
    ) -> Tableau {
        match binary.operator {
            BinaryOperator::Conjunction(_) => {
                self.build_conjunction(expr, &binary.lhs, &binary.rhs, expect)
            }
            BinaryOperator::Disjunction(_) => {
                self.build_disjunction(expr, &binary.lhs, &binary.rhs, expect)
            }
            BinaryOperator::ExclusiveDisjunction(_) => {
                self.build_exclusive(expr, &binary.lhs, &binary.rhs, expect)
            }
            BinaryOperator::Implication(_) => {
                self.build_implication(expr, &binary.lhs, &binary.rhs, expect)
            }
            BinaryOperator::Equivalence(_) => {
                self.build_equivalence(expr, &binary.lhs, &binary.rhs, expect)
            }
        }
    }

    #[must_use]
    pub fn build_expression(&self, expr: &Expression, expect: bool) -> Tableau {
        match expr.kind.as_ref() {
            ExpressionKind::TruthValue(value) => self.build_truth_value(expr, value, expect),
            ExpressionKind::Atomic(atomic) => self.build_atomic(expr, atomic, expect),
            ExpressionKind::Paren(paren) => self.build_paren(expr, paren, expect),
            ExpressionKind::Unary(unary) => self.build_unary(expr, unary, expect),
            ExpressionKind::Binary(binary) => self.build_binary(expr, binary, expect),
        }
    }
}

#[cfg(test)]
mod tests {}
