# easyparse

A basic parser combinators implementation in Rust that includes a complete toy language parser.

## Features

- **Parser Combinators**: A collection of composable parsing functions
- **Toy Language**: A simple arithmetic expression language with variables and assignments
- **Error Handling**: Descriptive error messages for parsing failures
- **Zero Dependencies**: Implemented using only the Rust standard library

## Parser Combinators

### Basic Parsers
- `char(c)` - Parse a specific character
- `string(s)` - Parse a specific string
- `digit()` - Parse a digit character (0-9)
- `letter()` - Parse a letter character
- `number()` - Parse an integer number
- `satisfy(predicate)` - Parse any character that satisfies a predicate

### Combinators
- `seq(p1, p2)` - Parse p1 followed by p2
- `choice(p1, p2)` - Try p1, if it fails try p2
- `map(parser, f)` - Transform the result of a parser
- `many(parser)` - Parse zero or more occurrences
- `many1(parser)` - Parse one or more occurrences
- `optional(parser)` - Parse zero or one occurrence
- `between(left, right, parser)` - Parse parser between left and right delimiters

## Toy Language

The toy language supports:

- **Numbers**: Integer literals (e.g., `42`, `123`)
- **Variables**: Identifiers starting with letter or underscore (e.g., `x`, `_var`, `counter`)
- **Arithmetic Operations**: `+`, `-`, `*`, `/` with proper precedence
- **Parentheses**: For grouping expressions
- **Assignments**: Variable assignment (e.g., `x = 42`)

### Language Grammar

```
statement   := assignment | expression
assignment  := identifier '=' expression
expression  := term (('+' | '-') term)*
term        := factor (('*' | '/') factor)*
factor      := number | identifier | '(' expression ')'
identifier  := (letter | '_') (alphanumeric | '_')*
number      := digit+
```

### Operator Precedence

1. Parentheses: `()`
2. Multiplication and Division: `*`, `/` (left-associative)
3. Addition and Subtraction: `+`, `-` (left-associative)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
easyparse = "0.1.0"
```

### Basic Parser Combinators

```rust
use easyparse::*;

// Parse a character
let result = char('a').parse("abc");
assert_eq!(result, Ok(('a', "bc")));

// Parse a number
let result = number().parse("123");
assert_eq!(result, Ok((123, "")));

// Combine parsers
let parser = seq(char('('), seq(number(), char(')')));
let result = parser.parse("(42)");
assert_eq!(result, Ok((('(', (42, ')')), "")));
```

### Toy Language Parser

```rust
use easyparse::*;

// Parse and evaluate expressions
let result = parse_and_evaluate("3 + 4 * 2");
assert_eq!(result.unwrap().1, 11); // 3 + (4 * 2) = 11

// Parse assignments
let result = parse_and_evaluate("x = 42");
assert_eq!(result.unwrap().1, 42);

// Complex expressions
let result = parse_and_evaluate("(10 + 5) / 3");
assert_eq!(result.unwrap().1, 5);
```

## Examples

### Running the Demo

```bash
cargo run
```

This will run a demo showing the parser combinators and toy language in action.

### Running Tests

```bash
cargo test
```

The test suite includes comprehensive tests for all parser combinators and the toy language.

## Architecture

The library is organized into three main modules:

- `parser`: Core parser types and traits
- `combinators`: Basic parser combinators
- `toy_language`: Implementation of the toy arithmetic language

### Parser Type

The core `Parser<T>` type wraps a function that takes input and returns either a successful parse result with the remaining input, or an error:

```rust
pub struct Parser<T> {
    parser: Rc<dyn for<'a> Fn(Input<'a>) -> ParseResult<'a, T>>,
}
```

### Error Handling

Parse errors include descriptive messages:

```rust
pub struct ParseError {
    pub message: String,
    pub position: usize,
}
```

## Contributing

Contributions are welcome! This is a learning project focused on demonstrating parser combinators in Rust.

## License

This project is in the public domain. Feel free to use it for learning and experimentation.
