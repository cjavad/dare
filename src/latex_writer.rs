use std::collections::HashMap;

use crate::{
    AtomicExpression, BinaryExpression, BinaryOperator, Error, Expression, ExpressionKind,
    ExpressionWriter, ParenExpression, Tableau, TableauWriter, TruthValueExpression,
    UnaryExpression, UnaryOperator,
};

#[derive(Clone, Debug, Default)]
pub struct LatexExpressionWriter {
    buffer: String,
}

impl LatexExpressionWriter {
    pub fn write_truth_value(&mut self, truth_value: &TruthValueExpression) {
        if truth_value.value == true {
            self.buffer += "T";
        } else {
            self.buffer += "F";
        }
    }

    pub fn write_atomic(&mut self, atomic: &AtomicExpression) {
        self.buffer += &atomic.ident.replace('_', "\\_");
    }

    pub fn write_paren(&mut self, paren: &ParenExpression) -> Result<(), Error> {
        self.buffer += "(";
        self.write_expression(&paren.expression)?;
        self.buffer += ")";

        Ok(())
    }

    pub fn write_unary(&mut self, unary: &UnaryExpression) -> Result<(), Error> {
        match unary.operator {
            UnaryOperator::Negation(_) => {
                self.buffer += "\\neg ";
            }
        }

        self.write_expression(&unary.operand)?;

        Ok(())
    }

    pub fn write_binary(&mut self, binary: &BinaryExpression) -> Result<(), Error> {
        self.write_expression(&binary.lhs)?;

        match binary.operator {
            BinaryOperator::Conjunction(_) => self.buffer += " \\land ",
            BinaryOperator::Disjunction(_) => self.buffer += " \\lor ",
            BinaryOperator::ExclusiveDisjunction(_) => self.buffer += " \\oplus ",
            BinaryOperator::Implication(_) => self.buffer += " \\to ",
            BinaryOperator::Equivalence(_) => self.buffer += " \\leftrightarrow ",
        }

        self.write_expression(&binary.rhs)?;

        Ok(())
    }

    pub fn finalize(self) -> String {
        format!("${}$", self.buffer)
    }
}

impl ExpressionWriter for LatexExpressionWriter {
    type Error = Error;

    fn write_expression(&mut self, expr: &Expression) -> Result<(), Self::Error> {
        match expr.kind.as_ref() {
            ExpressionKind::TruthValue(truth_value) => self.write_truth_value(truth_value),
            ExpressionKind::Atomic(atomic) => self.write_atomic(atomic),
            ExpressionKind::Paren(paren) => self.write_paren(paren)?,
            ExpressionKind::Unary(unary) => self.write_unary(unary)?,
            ExpressionKind::Binary(binary) => self.write_binary(binary)?,
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct LatexTableauWriter {
    x: f32,
    y: f32,
    branch_width: f32,
    branch_height: f32,
    width: f32,
    height: f32,
    has_dimensions: bool,
    next_id: u32,
    id_map: HashMap<u32, u32>,
    buffer: String,
}

impl Default for LatexTableauWriter {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            branch_width: 40.0,
            branch_height: 30.0,
            width: 0.0,
            height: 0.0,
            next_id: 1,
            id_map: HashMap::new(),
            has_dimensions: false,
            buffer: String::new(),
        }
    }
}

impl LatexTableauWriter {
    fn tableau_width(&self, tableau: &Tableau) -> f32 {
        tableau.width() as f32 * self.branch_width
    }

    fn tableau_height(&self, tableau: &Tableau) -> f32 {
        let mut height = 0.0;

        height += tableau.expectations.len() as f32 * 10.0;

        let mut max_height = 0.0f32;
        for branch in &tableau.branches {
            max_height = max_height.max(self.tableau_height(branch) + self.branch_height);
        }

        height += max_height;

        height
    }

    pub fn get_id(&mut self, id: u32) -> u32 {
        match self.id_map.get(&id) {
            Some(mapped_id) => *mapped_id,
            None => {
                let mapped_id = self.next_id;
                self.next_id += 1;
                self.id_map.insert(id, mapped_id);
                mapped_id
            }
        }
    }

    pub fn finalize(self) -> String {
        format!(
            "\\begin{{picture}}({}, {})\n{}\\end{{picture}}",
            self.width, self.height, self.buffer
        )
    }
}

impl TableauWriter for LatexTableauWriter {
    type Error = Error;

    fn write_tableau(&mut self, tableau: &Tableau) -> Result<(), Self::Error> {
        if !self.has_dimensions {
            self.width = self.tableau_width(tableau);
            self.height = self.tableau_height(tableau);
        }

        for expectation in &tableau.expectations {
            let mut writer = LatexExpressionWriter::default();
            writer.write_expression(&expectation.expr)?;

            let value = if expectation.truth_value == true {
                "T"
            } else {
                "F"
            };

            let left = format!(
                "\t\\put({}, {}){{\\makebox(0, 0)[r]{{{}}}}}\n",
                self.x - 4.0 + self.width / 2.0,
                self.y + self.height,
                writer.finalize()
            );

            self.buffer += &left;

            let terminates = expectation.expr.terminates();
            let right = if !tableau.solves_expectation(expectation.id) && !terminates {
                let id = format!(
                    "\\pdfliteral{{0.9, 0.37, 0.74, rg}} {} \\pdfliteral{{0, 0, 0, rg}}",
                    self.get_id(expectation.id),
                );

                format!(
                    "\t\\put({}, {}){{\\makebox(0, 0)[l]{{$: {} \\quad {}$}}}}\n",
                    self.x - 1.5 + self.width / 2.0,
                    self.y + self.height,
                    value,
                    id
                )
            } else {
                format!(
                    "\t\\put({}, {}){{\\makebox(0, 0)[l]{{$: {}$}}}}\n",
                    self.x - 1.5 + self.width / 2.0,
                    self.y + self.height,
                    value,
                )
            };

            self.buffer += &right;

            self.y -= 10.0;
        }

        for (i, branch) in tableau.branches.iter().enumerate() {
            let mut writer = LatexTableauWriter::default();

            let width = branch.width() as f32 * self.branch_width;
            let mut width_mod = i as f32 / (tableau.branches.len() - 1) as f32 * 2.0 - 1.0;

            if width_mod.is_nan() {
                width_mod = 0.0;
            }

            let calculated_width = self.tableau_width(tableau);
            width_mod *= 1.0 - width / calculated_width;

            writer.width = self.width;
            writer.height = self.height;
            writer.has_dimensions = true;
            writer.x = self.x + calculated_width * width_mod;
            writer.y = self.y - self.branch_height;
            writer.next_id = self.next_id;
            writer.id_map = self.id_map.clone();

            let mid_x = (writer.x + self.x) / 2.0 + self.width / 2.0;
            let mid_y = (writer.y + 8.0 + self.y) / 2.0 + self.height;

            self.buffer += &format!(
                "\t\\qbezier({}, {})({}, {})({}, {})\n",
                self.x + self.width / 2.0,
                self.y + self.height,
                mid_x,
                mid_y,
                writer.x + self.width / 2.0,
                writer.y + 8.0 + self.height,
            );

            if !tableau.has_expectation(branch.expectation) && i == tableau.branches.len() - 1 {
                let id_y = if tableau.branches.len() == 1 {
                    mid_y
                } else {
                    mid_y + 5.0
                };

                let id = format!(
                    "\\pdfliteral{{0.9, 0.37, 0.74, rg}} {} \\pdfliteral{{0, 0, 0, rg}}",
                    self.get_id(branch.expectation)
                );
                self.buffer += &format!(
                    "\t\\put({}, {}){{\\makebox(0, 0)[c]{{${}$}}}}\n",
                    mid_x + 10.0,
                    id_y,
                    id
                );
            }

            writer.write_tableau(branch)?;
            self.buffer += &writer.buffer;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
