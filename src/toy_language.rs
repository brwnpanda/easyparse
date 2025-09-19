//! A simple toy language implementation
//! 
//! This module implements a parser for a simple arithmetic expression language
//! that supports:
//! - Numbers (integers)
//! - Variables (identifiers)
//! - Basic arithmetic operations (+, -, *, /)
//! - Parentheses for grouping
//! - Variable assignments

use crate::parser::{Parser, Input};
use crate::combinators::*;
use std::collections::HashMap;

/// Abstract Syntax Tree for our toy language
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(i32),
    Variable(String),
    BinaryOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Assignment {
        var: String,
        expr: Box<Expr>,
    },
}

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Parse an identifier (variable name)
pub fn identifier() -> Parser<String> {
    map(
        seq(
            choice(letter(), char('_')), 
            many(choice(alphanumeric(), char('_')))
        ),
        |(first, rest)| {
            let mut name = String::new();
            name.push(first);
            name.extend(rest);
            name
        },
    )
}

/// Parse a primary expression (number, variable, or parenthesized expression)
pub fn primary() -> Parser<Expr> {
    choice(
        // Number
        map(number(), Expr::Number),
        choice(
            // Variable
            map(identifier(), Expr::Variable),
            // Parenthesized expression - use term instead of expression to avoid recursion
            between(
                seq(char('('), skip_whitespace()),
                seq(skip_whitespace(), char(')')),
                Parser::new(|input| term().parse(input)),
            ),
        ),
    )
}

/// Parse a factor (primary with potential multiplication/division)
pub fn factor() -> Parser<Expr> {
    let primary_parser = primary();
    Parser::new(move |input: Input| {
        let (mut left, mut remaining) = primary_parser.parse(input)?;
        
        loop {
            // Skip whitespace
            let (_, remaining_ws) = skip_whitespace().parse(remaining).unwrap_or(((), remaining));
            remaining = remaining_ws;
            
            // Try to parse * or /
            if let Ok((op_char, remaining_op)) = choice(char('*'), char('/')).parse(remaining) {
                remaining = remaining_op;
                
                // Skip whitespace after operator
                let (_, remaining_ws) = skip_whitespace().parse(remaining).unwrap_or(((), remaining));
                remaining = remaining_ws;
                
                // Parse the right operand
                let (right, remaining_right) = primary_parser.parse(remaining)?;
                remaining = remaining_right;
                
                let op = match op_char {
                    '*' => BinOp::Multiply,
                    '/' => BinOp::Divide,
                    _ => unreachable!(),
                };
                
                left = Expr::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        
        Ok((left, remaining))
    })
}

/// Parse a term (factor with potential addition/subtraction)
pub fn term() -> Parser<Expr> {
    let factor_parser = factor();
    Parser::new(move |input: Input| {
        let (mut left, mut remaining) = factor_parser.parse(input)?;
        
        loop {
            // Skip whitespace
            let (_, remaining_ws) = skip_whitespace().parse(remaining).unwrap_or(((), remaining));
            remaining = remaining_ws;
            
            // Try to parse + or -
            if let Ok((op_char, remaining_op)) = choice(char('+'), char('-')).parse(remaining) {
                remaining = remaining_op;
                
                // Skip whitespace after operator
                let (_, remaining_ws) = skip_whitespace().parse(remaining).unwrap_or(((), remaining));
                remaining = remaining_ws;
                
                // Parse the right operand
                let (right, remaining_right) = factor_parser.parse(remaining)?;
                remaining = remaining_right;
                
                let op = match op_char {
                    '+' => BinOp::Add,
                    '-' => BinOp::Subtract,
                    _ => unreachable!(),
                };
                
                left = Expr::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        
        Ok((left, remaining))
    })
}

/// Parse an expression (just term for now)
pub fn expression() -> Parser<Expr> {
    term()
}

/// Parse an assignment
pub fn assignment() -> Parser<Expr> {
    Parser::new(move |input: Input| {
        let (var_name, remaining1) = identifier().parse(input)?;
        
        // Skip whitespace
        let (_, remaining2) = skip_whitespace().parse(remaining1).unwrap_or(((), remaining1));
        
        // Parse '='
        let (_, remaining3) = char('=').parse(remaining2)?;
        
        // Skip whitespace
        let (_, remaining4) = skip_whitespace().parse(remaining3).unwrap_or(((), remaining3));
        
        // Parse the expression
        let (expr, remaining5) = term().parse(remaining4)?;
        
        Ok((
            Expr::Assignment {
                var: var_name,
                expr: Box::new(expr),
            },
            remaining5,
        ))
    })
}

/// Parse a complete statement (assignment or expression)
pub fn statement() -> Parser<Expr> {
    Parser::new(move |input: Input| {
        // Skip leading whitespace
        let (_, remaining1) = skip_whitespace().parse(input).unwrap_or(((), input));
        
        // Try assignment first
        if let Ok((result, remaining)) = assignment().parse(remaining1) {
            // Skip trailing whitespace
            let (_, remaining3) = skip_whitespace().parse(remaining).unwrap_or(((), remaining));
            return Ok((result, remaining3));
        }
        
        // Fall back to expression
        let (expr, remaining2) = expression().parse(remaining1)?;
        
        // Skip trailing whitespace
        let (_, remaining3) = skip_whitespace().parse(remaining2).unwrap_or(((), remaining2));
        
        Ok((expr, remaining3))
    })
}

/// Evaluate an expression with a given variable context
pub fn evaluate(expr: &Expr, context: &mut HashMap<String, i32>) -> Result<i32, String> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Variable(name) => {
            context.get(name).copied()
                .ok_or_else(|| format!("Undefined variable: {}", name))
        }
        Expr::BinaryOp { op, left, right } => {
            let left_val = evaluate(left, context)?;
            let right_val = evaluate(right, context)?;
            
            match op {
                BinOp::Add => Ok(left_val + right_val),
                BinOp::Subtract => Ok(left_val - right_val),
                BinOp::Multiply => Ok(left_val * right_val),
                BinOp::Divide => {
                    if right_val == 0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(left_val / right_val)
                    }
                }
            }
        }
        Expr::Assignment { var, expr } => {
            let value = evaluate(expr, context)?;
            context.insert(var.clone(), value);
            Ok(value)
        }
    }
}

/// Parse and evaluate a string
pub fn parse_and_evaluate(input: &str) -> Result<(Expr, i32), String> {
    let mut context = HashMap::new();
    
    match statement().parse(input) {
        Ok((expr, remaining)) => {
            if !remaining.trim().is_empty() {
                return Err(format!("Unexpected input after expression: '{}'", remaining));
            }
            
            let value = evaluate(&expr, &mut context)?;
            Ok((expr, value))
        }
        Err(e) => Err(e.message),
    }
}