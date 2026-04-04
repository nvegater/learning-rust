// Step 4: Error Handling
//
// TS: try { parse(input) } catch (e) { ... }
// Rust: parse(input) returns Result<T, E> — you MUST handle it or the code won't compile.
//
// Two key types:
//   Option<T>  = Some(value) | None        — like T | undefined
//   Result<T, E> = Ok(value) | Err(error)  — like a discriminated union you can't ignore

use std::fmt;

// -- Custom error type --
// TS equivalent: class ParseError extends Error { kind: "empty" | "negative" | ... }
// Rust: use an enum (just like Step 3!)
#[derive(Debug, PartialEq)]
enum ParseError {
    Empty,
    NotANumber(String),
    Negative(f64),
    TooLarge(f64),
}

// Implement Display so our error can be printed nicely
// TS equivalent: toString() method
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "input was empty"),
            ParseError::NotANumber(s) => write!(f, "'{s}' is not a valid number"),
            ParseError::Negative(n) => write!(f, "{n} is negative"),
            ParseError::TooLarge(n) => write!(f, "{n} exceeds maximum (1000)"),
        }
    }
}

// -- Function that returns Result --
// TS: function parsePositive(input: string): number  (throws on error)
// Rust: returns Result — caller decides how to handle failure
fn parse_positive(input: &str) -> Result<f64, ParseError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(ParseError::Empty);
    }

    // str::parse returns Result<f64, ParseFloatError>
    // .map_err transforms the error type (like .catch(e => new MyError(e)))
    let number: f64 = trimmed
        .parse()
        .map_err(|_| ParseError::NotANumber(trimmed.to_string()))?;
    //                                                             ^ the ? operator
    // ? = "if Err, return early with that error; if Ok, unwrap the value"
    // It's like:  if (result.isErr()) return result;  but built into the language

    if number < 0.0 {
        return Err(ParseError::Negative(number));
    }

    if number > 1000.0 {
        return Err(ParseError::TooLarge(number));
    }

    Ok(number)
}

// -- Option<T> --
// TS: function findFirst(items: number[], predicate): number | undefined
// Rust: Option<T> instead of returning undefined
fn first_above(values: &[f64], threshold: f64) -> Option<f64> {
    // .iter() = like array.values()
    // .find() returns Option<&f64>
    // .copied() converts &f64 to f64 (dereferences)
    values.iter().copied().find(|&v| v > threshold)
}

// -- Chaining Results with ? --
// When multiple operations can fail, ? chains them cleanly.
// TS equivalent would be nested try/catch or a chain of if/else checks.
fn parse_and_double(input: &str) -> Result<f64, ParseError> {
    let value = parse_positive(input)?; // early return if Err
    Ok(value * 2.0)
}

// -- Converting Option to Result --
// Sometimes you have Option but need Result (to add error context)
fn first_above_or_error(values: &[f64], threshold: f64) -> Result<f64, String> {
    // .ok_or() converts None → Err, Some → Ok
    first_above(values, threshold).ok_or(format!("no value above {threshold}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Result: happy path ---

    #[test]
    fn parse_valid_number() {
        assert_eq!(parse_positive("42"), Ok(42.0));
    }

    #[test]
    fn parse_with_whitespace() {
        assert_eq!(parse_positive("  7.5  "), Ok(7.5));
    }

    // --- Result: error paths ---
    // In TS you'd write: expect(() => parse("")).toThrow()
    // In Rust you match on the Err variant directly — no catching needed

    #[test]
    fn parse_empty() {
        assert_eq!(parse_positive(""), Err(ParseError::Empty));
    }

    #[test]
    fn parse_not_a_number() {
        assert_eq!(
            parse_positive("abc"),
            Err(ParseError::NotANumber("abc".to_string()))
        );
    }

    #[test]
    fn parse_negative() {
        assert_eq!(parse_positive("-5"), Err(ParseError::Negative(-5.0)));
    }

    #[test]
    fn parse_too_large() {
        assert_eq!(parse_positive("9999"), Err(ParseError::TooLarge(9999.0)));
    }

    // --- Error display ---

    #[test]
    fn error_messages_are_readable() {
        // .to_string() calls Display::fmt — like toString() in TS
        assert_eq!(ParseError::Empty.to_string(), "input was empty");
        assert_eq!(
            ParseError::NotANumber("abc".into()).to_string(),
            "'abc' is not a valid number"
        );
    }

    // --- The ? operator chains errors ---

    #[test]
    fn parse_and_double_happy() {
        assert_eq!(parse_and_double("5"), Ok(10.0));
    }

    #[test]
    fn parse_and_double_propagates_error() {
        // The ? in parse_and_double passes through the error from parse_positive
        assert_eq!(parse_and_double(""), Err(ParseError::Empty));
    }

    // --- Option<T> ---

    #[test]
    fn find_above_some() {
        let values = vec![1.0, 5.0, 10.0];
        assert_eq!(first_above(&values, 3.0), Some(5.0));
    }

    #[test]
    fn find_above_none() {
        let values = vec![1.0, 2.0, 3.0];
        assert_eq!(first_above(&values, 100.0), None);
    }

    // --- Option → Result conversion ---

    #[test]
    fn option_to_result_ok() {
        let values = vec![1.0, 5.0, 10.0];
        assert_eq!(first_above_or_error(&values, 3.0), Ok(5.0));
    }

    #[test]
    fn option_to_result_err() {
        let values = vec![1.0, 2.0];
        assert_eq!(
            first_above_or_error(&values, 100.0),
            Err("no value above 100".to_string())
        );
    }
}
