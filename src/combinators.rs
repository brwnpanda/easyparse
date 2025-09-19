//! Basic parser combinators

use crate::parser::{Input, Parser, parse_error};

/// Parse a specific character
pub fn char(expected: char) -> Parser<char> {
    Parser::new(move |input: Input| {
        if let Some(first) = input.chars().next() {
            if first == expected {
                let remaining = &input[first.len_utf8()..];
                Ok((first, remaining))
            } else {
                Err(parse_error(&format!("Expected '{}', found '{}'", expected, first), input))
            }
        } else {
            Err(parse_error(&format!("Expected '{}', found EOF", expected), input))
        }
    })
}

/// Parse a specific string
pub fn string(expected: &'static str) -> Parser<String> {
    Parser::new(move |input: Input| {
        if input.starts_with(expected) {
            let remaining = &input[expected.len()..];
            Ok((expected.to_string(), remaining))
        } else {
            Err(parse_error(&format!("Expected '{}'", expected), input))
        }
    })
}

/// Parse any character that satisfies a predicate
pub fn satisfy<F>(predicate: F) -> Parser<char>
where
    F: Fn(char) -> bool + 'static,
{
    Parser::new(move |input: Input| {
        if let Some(first) = input.chars().next() {
            if predicate(first) {
                let remaining = &input[first.len_utf8()..];
                Ok((first, remaining))
            } else {
                Err(parse_error("Character doesn't satisfy predicate", input))
            }
        } else {
            Err(parse_error("Unexpected EOF", input))
        }
    })
}

/// Parse a digit character
pub fn digit() -> Parser<char> {
    satisfy(|c| c.is_ascii_digit())
}

/// Parse a letter character
pub fn letter() -> Parser<char> {
    satisfy(|c| c.is_alphabetic())
}

/// Parse an alphanumeric character
pub fn alphanumeric() -> Parser<char> {
    satisfy(|c| c.is_alphanumeric())
}

/// Parse whitespace characters
pub fn whitespace() -> Parser<char> {
    satisfy(|c| c.is_whitespace())
}

/// Sequence combinator - parse first, then second
pub fn seq<T, U>(parser1: Parser<T>, parser2: Parser<U>) -> Parser<(T, U)>
where
    T: 'static,
    U: 'static,
{
    Parser::new(move |input: Input| {
        let (result1, remaining1) = parser1.parse(input)?;
        let (result2, remaining2) = parser2.parse(remaining1)?;
        Ok(((result1, result2), remaining2))
    })
}

/// Choice combinator - try first parser, if it fails try second
pub fn choice<T>(parser1: Parser<T>, parser2: Parser<T>) -> Parser<T>
where
    T: 'static,
{
    Parser::new(move |input: Input| {
        match parser1.parse(input) {
            Ok(result) => Ok(result),
            Err(_) => parser2.parse(input),
        }
    })
}

/// Map combinator - transform the result of a parser
pub fn map<T, U, F>(parser: Parser<T>, f: F) -> Parser<U>
where
    T: 'static,
    U: 'static,
    F: Fn(T) -> U + 'static,
{
    Parser::new(move |input: Input| {
        let (result, remaining) = parser.parse(input)?;
        Ok((f(result), remaining))
    })
}

/// Many combinator - parse zero or more occurrences
pub fn many<T>(parser: Parser<T>) -> Parser<Vec<T>>
where
    T: 'static,
{
    Parser::new(move |mut input: Input| {
        let mut results = Vec::new();
        
        while let Ok((result, remaining)) = parser.parse(input) {
            results.push(result);
            input = remaining;
        }
        
        Ok((results, input))
    })
}

/// Many1 combinator - parse one or more occurrences
pub fn many1<T>(parser: Parser<T>) -> Parser<Vec<T>>
where
    T: 'static,
{
    Parser::new(move |input: Input| {
        let (first, mut remaining) = parser.parse(input)?;
        let mut results = vec![first];
        
        // Parse remaining occurrences
        while let Ok((result, new_remaining)) = parser.parse(remaining) {
            results.push(result);
            remaining = new_remaining;
        }
        
        Ok((results, remaining))
    })
}

/// Optional combinator - parse zero or one occurrence
pub fn optional<T>(parser: Parser<T>) -> Parser<Option<T>>
where
    T: 'static,
{
    Parser::new(move |input: Input| {
        match parser.parse(input) {
            Ok((result, remaining)) => Ok((Some(result), remaining)),
            Err(_) => Ok((None, input)),
        }
    })
}

/// Skip whitespace
pub fn skip_whitespace() -> Parser<()> {
    map(many(whitespace()), |_| ())
}

/// Parse a number (integer)
pub fn number() -> Parser<i32> {
    map(many1(digit()), |digits| {
        let num_str: String = digits.into_iter().collect();
        num_str.parse().unwrap_or(0)
    })
}

/// Parse between delimiters
pub fn between<T, L, R>(left: Parser<L>, right: Parser<R>, parser: Parser<T>) -> Parser<T>
where
    T: 'static,
    L: 'static,
    R: 'static,
{
    Parser::new(move |input: Input| {
        let (_, remaining1) = left.parse(input)?;
        let (result, remaining2) = parser.parse(remaining1)?;
        let (_, remaining3) = right.parse(remaining2)?;
        Ok((result, remaining3))
    })
}