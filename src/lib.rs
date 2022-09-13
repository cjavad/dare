#![deny(unsafe_op_in_unsafe_fn)]

//! Dare is a crate for parsing and solving logical expressions.

mod ast;
mod error;
mod interpreter;
mod parser;
mod span;
mod tableau;
mod token;
mod token_stream;

pub use ast::*;
pub use error::*;
pub use interpreter::*;
pub use parser::*;
pub use span::*;
pub use tableau::*;
pub use token::*;
pub use token_stream::*;
