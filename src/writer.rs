use crate::{Expression, Tableau};

pub trait ExpressionWriter {
    type Error;

    fn write_expression(&mut self, expr: &Expression) -> Result<(), Self::Error>;
}

pub trait TableauWriter {
    type Error;

    fn write_tableau(&mut self, tableau: &Tableau) -> Result<(), Self::Error>;
}
