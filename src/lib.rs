#![deny(unsafe_op_in_unsafe_fn)]

//! Dare is a crate for parsing and solving logical expressions.

mod ast;
mod error;
mod span;
mod token;
mod token_stream;

pub use ast::*;
pub use error::*;
pub use span::*;
pub use token::*;
pub use token_stream::*;
