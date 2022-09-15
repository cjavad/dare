use std::ops::{Deref, DerefMut};

use crate::{
    AtomicExpression, BinaryExpression, BinaryOperator, Error, Expression, ExpressionKind,
    ParenExpression, Parser, TruthValueExpression, UnaryExpression, UnaryOperator,
};

#[derive(Clone, Debug)]
pub struct Expectation {
    pub expr: Expression,
    pub truth_value: bool,
    pub id: u32,
}

impl Expectation {
    pub const fn new(expr: Expression, truth_value: bool, id: u32) -> Self {
        Self {
            expr,
            truth_value,
            id,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TableauBranch {
    pub tableau: Tableau,
    pub expectation: u32,
}

impl TableauBranch {
    pub fn new(tableau: Tableau, expectation: u32) -> Self {
        Self {
            tableau,
            expectation,
        }
    }
}

impl Deref for TableauBranch {
    type Target = Tableau;

    fn deref(&self) -> &Self::Target {
        &self.tableau
    }
}

impl DerefMut for TableauBranch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tableau
    }
}

#[derive(Clone, Debug)]
pub struct Tableau {
    pub expectations: Vec<Expectation>,
    pub branches: Vec<TableauBranch>,
}

impl Tableau {
    pub fn parse(source: &str, expect: bool) -> Result<Self, Error> {
        let mut builder = TableauBuilder::default();
        let parser = Parser::default();
        Ok(builder.build_expression(&parser.parse(source)?, expect))
    }

    pub fn has_expectation(&self, id: u32) -> bool {
        self.expectations.iter().any(|e| e.id == id)
    }

    pub fn solves_expectation(&self, id: u32) -> bool {
        self.branches.iter().any(|b| b.expectation == id)
    }

    pub fn merge(&mut self, mut other: Self) {
        self.expectations.append(&mut other.expectations);
        self.branches.append(&mut other.branches);
    }

    pub fn append(&mut self, mut other: TableauBranch) {
        self.expectations.append(&mut other.tableau.expectations);

        if self.branches.is_empty() {
            self.branches = other.tableau.branches;

            return;
        }

        for branch in self.branches.iter_mut() {
            branch.tableau.append(other.clone());
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
pub struct TableauBuilder {
    id: u32,
}

impl TableauBuilder {
    pub fn next_id(&mut self) -> u32 {
        let id = self.id;
        self.id += 1;
        id
    }

    #[must_use]
    pub fn build_truth_value(
        &mut self,
        expr: &Expression,
        _truth_value: &TruthValueExpression,
        expect: bool,
    ) -> Tableau {
        Tableau {
            expectations: vec![Expectation::new(expr.clone(), expect, self.next_id())],
            branches: Vec::new(),
        }
    }

    #[must_use]
    pub fn build_atomic(
        &mut self,
        expr: &Expression,
        _atomic: &AtomicExpression,
        expect: bool,
    ) -> Tableau {
        Tableau {
            expectations: vec![Expectation::new(expr.clone(), expect, self.next_id())],
            branches: Vec::new(),
        }
    }

    #[must_use]
    pub fn build_paren(
        &mut self,
        _expr: &Expression,
        paren: &ParenExpression,
        expect: bool,
    ) -> Tableau {
        self.build_expression(&paren.expression, expect)
    }

    #[must_use]
    pub fn build_negation(&mut self, operand: &Expression, expect: bool) -> Tableau {
        self.build_expression(&operand, !expect)
    }

    #[must_use]
    pub fn build_unary(
        &mut self,
        expr: &Expression,
        unary: &UnaryExpression,
        expect: bool,
    ) -> Tableau {
        let id = self.next_id();

        let tableau = match unary.operator {
            UnaryOperator::Negation(_) => self.build_expression(&unary.operand, !expect),
        };

        Tableau {
            expectations: vec![Expectation::new(expr.clone(), expect, id)],
            branches: vec![TableauBranch::new(tableau, id)],
        }
    }

    #[must_use]
    pub fn build_conjunction(
        &mut self,
        expr: &Expression,
        lhs: &Expression,
        rhs: &Expression,
        expect: bool,
    ) -> Tableau {
        let id = self.next_id();

        if expect == true {
            let mut lhs_tableau = self.build_expression(lhs, true);
            let rhs_tableau = self.build_expression(rhs, true);

            lhs_tableau.append(TableauBranch::new(rhs_tableau, id));

            Tableau {
                expectations: vec![Expectation::new(expr.clone(), true, id)],
                branches: vec![TableauBranch::new(lhs_tableau, id)],
            }
        } else {
            let lhs_tableau = self.build_expression(lhs, false);
            let rhs_tableau = self.build_expression(rhs, false);

            Tableau {
                expectations: vec![Expectation::new(expr.clone(), false, id)],
                branches: vec![
                    TableauBranch::new(lhs_tableau, id),
                    TableauBranch::new(rhs_tableau, id),
                ],
            }
        }
    }

    #[must_use]
    pub fn build_disjunction(
        &mut self,
        expr: &Expression,
        lhs: &Expression,
        rhs: &Expression,
        expect: bool,
    ) -> Tableau {
        let id = self.next_id();

        if expect == true {
            let lhs_tableau = self.build_expression(lhs, true);
            let rhs_tableau = self.build_expression(rhs, true);

            Tableau {
                expectations: vec![Expectation::new(expr.clone(), true, id)],
                branches: vec![
                    TableauBranch::new(lhs_tableau, id),
                    TableauBranch::new(rhs_tableau, id),
                ],
            }
        } else {
            let mut lhs_tableau = self.build_expression(lhs, false);
            let rhs_tableau = self.build_expression(rhs, false);

            lhs_tableau.append(TableauBranch::new(rhs_tableau, id));

            Tableau {
                expectations: vec![Expectation::new(expr.clone(), true, id)],
                branches: vec![TableauBranch::new(lhs_tableau, id)],
            }
        }
    }

    #[must_use]
    pub fn build_exclusive(
        &mut self,
        expr: &Expression,
        lhs: &Expression,
        rhs: &Expression,
        expect: bool,
    ) -> Tableau {
        self.build_equivalence(expr, lhs, rhs, !expect)
    }

    #[must_use]
    pub fn build_implication(
        &mut self,
        expr: &Expression,
        lhs: &Expression,
        rhs: &Expression,
        expect: bool,
    ) -> Tableau {
        let id = self.next_id();

        if expect == true {
            let lhs_tableau = self.build_expression(lhs, false);
            let rhs_tableau = self.build_expression(rhs, true);

            Tableau {
                expectations: vec![Expectation::new(expr.clone(), true, id)],
                branches: vec![
                    TableauBranch::new(lhs_tableau, id),
                    TableauBranch::new(rhs_tableau, id),
                ],
            }
        } else {
            let mut lhs_tableau = self.build_expression(lhs, true);
            let rhs_tableau = self.build_expression(rhs, false);

            lhs_tableau.append(TableauBranch::new(rhs_tableau, id));

            Tableau {
                expectations: vec![Expectation::new(expr.clone(), false, id)],
                branches: vec![TableauBranch::new(lhs_tableau, id)],
            }
        }
    }

    #[must_use]
    pub fn build_equivalence(
        &mut self,
        expr: &Expression,
        lhs: &Expression,
        rhs: &Expression,
        expect: bool,
    ) -> Tableau {
        let id = self.next_id();

        if expect == true {
            let mut lhs_a = self.build_expression(lhs, true);
            let rhs_a = self.build_expression(rhs, true);

            lhs_a.append(TableauBranch::new(rhs_a, id));

            let mut lhs_b = self.build_expression(lhs, false);
            let rhs_b = self.build_expression(rhs, false);

            lhs_b.append(TableauBranch::new(rhs_b, id));

            Tableau {
                expectations: vec![Expectation::new(expr.clone(), true, id)],
                branches: vec![TableauBranch::new(lhs_a, id), TableauBranch::new(lhs_b, id)],
            }
        } else {
            let mut lhs_a = self.build_expression(lhs, true);
            let rhs_a = self.build_expression(rhs, false);

            lhs_a.append(TableauBranch::new(rhs_a, id));

            let mut lhs_b = self.build_expression(lhs, false);
            let rhs_b = self.build_expression(rhs, true);

            lhs_b.append(TableauBranch::new(rhs_b, id));

            Tableau {
                expectations: vec![Expectation::new(expr.clone(), false, id)],
                branches: vec![TableauBranch::new(lhs_a, id), TableauBranch::new(lhs_b, id)],
            }
        }
    }

    #[must_use]
    pub fn build_binary(
        &mut self,
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
    pub fn build_expression(&mut self, expr: &Expression, expect: bool) -> Tableau {
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
