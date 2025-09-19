//! Core parser types and traits

use std::fmt;
use std::rc::Rc;

/// The input to a parser - just a string slice for simplicity
pub type Input<'a> = &'a str;

/// The result of a parsing operation
pub type ParseResult<'a, T> = Result<(T, Input<'a>), ParseError>;

/// Error type for parsing failures
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub position: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error at position {}: {}", self.position, self.message)
    }
}

impl std::error::Error for ParseError {}

/// A parser function type
pub type ParseFn<T> = Rc<dyn for<'a> Fn(Input<'a>) -> ParseResult<'a, T>>;

/// A parser is essentially a function that takes input and returns a result
#[derive(Clone)]
pub struct Parser<T> {
    parser: ParseFn<T>,
}

impl<T> Parser<T> {
    pub fn new<F>(f: F) -> Self
    where
        F: for<'a> Fn(Input<'a>) -> ParseResult<'a, T> + 'static,
    {
        Self {
            parser: Rc::new(f),
        }
    }

    pub fn parse<'a>(&self, input: Input<'a>) -> ParseResult<'a, T> {
        (self.parser)(input)
    }
}

/// Helper function to create parse errors
pub fn parse_error(message: &str, _input: Input) -> ParseError {
    ParseError {
        message: message.to_string(),
        position: 0, // Simplified - in a real implementation we'd track position
    }
}