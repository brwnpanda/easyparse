//! A simple parser combinator library for Rust
//! 
//! This library provides basic parser combinators that can be composed
//! to build complex parsers for various languages and formats.

pub mod parser;
pub mod combinators;
pub mod toy_language;

pub use parser::*;
pub use combinators::*;
pub use toy_language::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_char_parser() {
        let parser = char('a');
        
        // Test successful parse
        let result = parser.parse("abc");
        assert_eq!(result, Ok(('a', "bc")));
        
        // Test failure
        let result = parser.parse("xyz");
        assert!(result.is_err());
        
        // Test empty input
        let result = parser.parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_string_parser() {
        let parser = string("hello");
        
        // Test successful parse
        let result = parser.parse("hello world");
        assert_eq!(result, Ok(("hello".to_string(), " world")));
        
        // Test failure
        let result = parser.parse("hi there");
        assert!(result.is_err());
        
        // Test exact match
        let result = parser.parse("hello");
        assert_eq!(result, Ok(("hello".to_string(), "")));
    }

    #[test]
    fn test_digit_parser() {
        let parser = digit();
        
        // Test successful parse
        let result = parser.parse("123");
        assert_eq!(result, Ok(('1', "23")));
        
        // Test failure
        let result = parser.parse("abc");
        assert!(result.is_err());
    }

    #[test]
    fn test_number_parser() {
        let parser = number();
        
        // Test single digit
        let result = parser.parse("5");
        assert_eq!(result, Ok((5, "")));
        
        // Test multiple digits
        let result = parser.parse("123abc");
        assert_eq!(result, Ok((123, "abc")));
        
        // Test failure
        let result = parser.parse("abc");
        assert!(result.is_err());
    }

    #[test]
    fn test_seq_combinator() {
        let parser = seq(char('a'), char('b'));
        
        // Test successful parse
        let result = parser.parse("abc");
        assert_eq!(result, Ok((('a', 'b'), "c")));
        
        // Test partial failure
        let result = parser.parse("axc");
        assert!(result.is_err());
    }

    #[test]
    fn test_choice_combinator() {
        let parser = choice(char('a'), char('b'));
        
        // Test first choice
        let result = parser.parse("abc");
        assert_eq!(result, Ok(('a', "bc")));
        
        // Test second choice
        let result = parser.parse("bac");
        assert_eq!(result, Ok(('b', "ac")));
        
        // Test failure
        let result = parser.parse("xyz");
        assert!(result.is_err());
    }

    #[test]
    fn test_many_combinator() {
        let parser = many(char('a'));
        
        // Test multiple matches
        let result = parser.parse("aaab");
        assert_eq!(result, Ok((vec!['a', 'a', 'a'], "b")));
        
        // Test no matches
        let result = parser.parse("bbb");
        assert_eq!(result, Ok((vec![], "bbb")));
        
        // Test empty input
        let result = parser.parse("");
        assert_eq!(result, Ok((vec![], "")));
    }

    #[test]
    fn test_many1_combinator() {
        let parser = many1(char('a'));
        
        // Test multiple matches
        let result = parser.parse("aaab");
        assert_eq!(result, Ok((vec!['a', 'a', 'a'], "b")));
        
        // Test single match
        let result = parser.parse("ab");
        assert_eq!(result, Ok((vec!['a'], "b")));
        
        // Test no matches (should fail)
        let result = parser.parse("bbb");
        assert!(result.is_err());
    }

    #[test]
    fn test_optional_combinator() {
        let parser = optional(char('a'));
        
        // Test present
        let result = parser.parse("abc");
        assert_eq!(result, Ok((Some('a'), "bc")));
        
        // Test absent
        let result = parser.parse("xyz");
        assert_eq!(result, Ok((None, "xyz")));
    }

    #[test]
    fn test_map_combinator() {
        let parser = map(char('a'), |c| c.to_uppercase().to_string());
        
        let result = parser.parse("abc");
        assert_eq!(result, Ok(("A".to_string(), "bc")));
    }

    #[test]
    fn test_number_expression() {
        let result = parse_and_evaluate("42");
        assert_eq!(result, Ok((Expr::Number(42), 42)));
    }

    #[test]
    fn test_addition() {
        let result = parse_and_evaluate("3 + 4");
        let expected = Expr::BinaryOp {
            op: BinOp::Add,
            left: Box::new(Expr::Number(3)),
            right: Box::new(Expr::Number(4)),
        };
        assert_eq!(result, Ok((expected, 7)));
    }

    #[test]
    fn test_subtraction() {
        let result = parse_and_evaluate("10 - 3");
        let expected = Expr::BinaryOp {
            op: BinOp::Subtract,
            left: Box::new(Expr::Number(10)),
            right: Box::new(Expr::Number(3)),
        };
        assert_eq!(result, Ok((expected, 7)));
    }

    #[test]
    fn test_multiplication() {
        let result = parse_and_evaluate("5 * 6");
        let expected = Expr::BinaryOp {
            op: BinOp::Multiply,
            left: Box::new(Expr::Number(5)),
            right: Box::new(Expr::Number(6)),
        };
        assert_eq!(result, Ok((expected, 30)));
    }

    #[test]
    fn test_division() {
        let result = parse_and_evaluate("15 / 3");
        let expected = Expr::BinaryOp {
            op: BinOp::Divide,
            left: Box::new(Expr::Number(15)),
            right: Box::new(Expr::Number(3)),
        };
        assert_eq!(result, Ok((expected, 5)));
    }

    #[test]
    fn test_assignment() {
        let result = parse_and_evaluate("x = 42");
        let expected = Expr::Assignment {
            var: "x".to_string(),
            expr: Box::new(Expr::Number(42)),
        };
        assert_eq!(result, Ok((expected, 42)));
    }

    #[test]
    fn test_precedence() {
        // Test that multiplication has higher precedence than addition
        let result = parse_and_evaluate("2 + 3 * 4");
        match result {
            Ok((Expr::BinaryOp { op: BinOp::Add, left, right }, value)) => {
                assert_eq!(left.as_ref(), &Expr::Number(2));
                match right.as_ref() {
                    Expr::BinaryOp { op: BinOp::Multiply, left: mul_left, right: mul_right } => {
                        assert_eq!(mul_left.as_ref(), &Expr::Number(3));
                        assert_eq!(mul_right.as_ref(), &Expr::Number(4));
                    }
                    _ => panic!("Expected multiplication on right side"),
                }
                assert_eq!(value, 14); // 2 + (3 * 4) = 14
            }
            _ => panic!("Expected addition with multiplication"),
        }
    }

    #[test]
    fn test_evaluation_with_context() {
        let expr = Expr::Variable("x".to_string());
        let mut context = HashMap::new();
        context.insert("x".to_string(), 42);
        
        let result = evaluate(&expr, &mut context);
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_undefined_variable() {
        let expr = Expr::Variable("y".to_string());
        let mut context = HashMap::new();
        
        let result = evaluate(&expr, &mut context);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Undefined variable: y");
    }

    #[test]
    fn test_division_by_zero() {
        let result = parse_and_evaluate("5 / 0");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Division by zero"));
    }

    #[test]
    fn test_whitespace_handling() {
        let result = parse_and_evaluate("  3   +   4  ");
        let expected = Expr::BinaryOp {
            op: BinOp::Add,
            left: Box::new(Expr::Number(3)),
            right: Box::new(Expr::Number(4)),
        };
        assert_eq!(result, Ok((expected, 7)));
    }

    #[test]
    fn test_identifier_parsing() {
        let result = identifier().parse("variable123_name");
        assert_eq!(result, Ok(("variable123_name".to_string(), "")));
        
        let result = identifier().parse("_private");
        assert_eq!(result, Ok(("_private".to_string(), "")));
    }

    #[test]
    fn test_complex_expression() {
        let result = parse_and_evaluate("10 - 2 * 3");
        // Should be 10 - (2 * 3) = 4
        assert_eq!(result.unwrap().1, 4);
    }
}