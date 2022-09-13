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

#[derive(Clone, Debug, Default)]
pub struct LatexTableauWriter {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    has_dimensions: bool,
    buffer: String,
}

impl LatexTableauWriter {
    fn tableau_width(tableau: &Tableau) -> f32 {
        tableau.width() as f32 * 40.0
    }

    fn tableau_height(tableau: &Tableau) -> f32 {
        let mut height = 0.0;

        height += tableau.expectations.len() as f32 * 10.0;

        for branch in &tableau.branches {
            height += Self::tableau_height(branch);
        }

        height
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
            self.width = Self::tableau_width(tableau);
            self.height = Self::tableau_height(tableau);
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
            let right = format!(
                "\t\\put({}, {}){{\\makebox(0, 0)[l]{{$: {}$}}}}\n",
                self.x - 1.5 + self.width / 2.0,
                self.y + self.height,
                value
            );

            self.buffer += &left;
            self.buffer += &right;

            self.y -= 10.0;
        }

        for (i, branch) in tableau.branches.iter().enumerate() {
            let mut writer = LatexTableauWriter::default();

            let width = branch.width() as f32 * 40.0;
            let mut width_mod = i as f32 / (tableau.branches.len() - 1) as f32 * 2.0 - 1.0;

            if width_mod.is_nan() {
                width_mod = 0.0;
            }

            let calculated_width = Self::tableau_width(tableau);
            width_mod *= 1.0 - width / calculated_width;

            writer.width = self.width;
            writer.height = self.height;
            writer.has_dimensions = true;
            writer.x = self.x + calculated_width * width_mod;
            writer.y = self.y - 20.0;

            self.buffer += &format!(
                "\t\\qbezier({}, {})({}, {})({}, {})\n",
                self.x + self.width / 2.0,
                self.y + self.height,
                (writer.x + self.x) / 2.0 + self.width / 2.0,
                (writer.y + 8.0 + self.y) / 2.0 + self.height,
                writer.x + self.width / 2.0,
                writer.y + 8.0 + self.height,
            );

            writer.write_tableau(branch)?;
            self.buffer += &writer.buffer;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
