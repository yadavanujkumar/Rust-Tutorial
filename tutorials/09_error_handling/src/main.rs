// ============================================================
// Tutorial 09: Error Handling
// ============================================================
//
// Rust distinguishes between two categories of errors:
//   - Unrecoverable errors: `panic!` — program stops
//   - Recoverable errors:   `Result<T, E>` — caller decides what to do
//
// Key concepts covered:
//   - panic! and unwrap/expect (for unrecoverable errors)
//   - The Result<T, E> enum
//   - Matching on Result
//   - Propagating errors with the `?` operator
//   - Custom error types
//   - Converting between error types

use std::fmt;
use std::num::ParseIntError;

fn main() {
    // ----------------------------------------------------------
    // panic! — Unrecoverable Errors
    // ----------------------------------------------------------
    // In production code, only use panic! for bugs (logic errors).
    // panic!("this would crash the program");

    // Out-of-bounds access panics automatically:
    // let v = vec![1, 2, 3];
    // let _ = v[99]; // panics: index out of bounds

    // ----------------------------------------------------------
    // Result<T, E>
    // ----------------------------------------------------------
    // Result is defined as:
    //   enum Result<T, E> {
    //       Ok(T),    // success: contains value of type T
    //       Err(E),   // failure: contains error of type E
    //   }

    // Parsing a string as a number returns a Result
    let good: Result<i32, _> = "42".parse();
    let bad:  Result<i32, _> = "abc".parse();

    println!("good parse: {:?}", good); // Ok(42)
    println!("bad parse:  {:?}", bad);  // Err(...)

    // ----------------------------------------------------------
    // Matching on Result
    // ----------------------------------------------------------
    match &good {
        Ok(n)  => println!("Parsed successfully: {}", n),
        Err(e) => println!("Parse failed: {}", e),
    }

    // ----------------------------------------------------------
    // unwrap and expect
    // ----------------------------------------------------------
    // unwrap() returns the value or panics with a default message.
    let n = good.unwrap();
    println!("unwrap: {}", n);

    // expect() lets you provide a custom panic message.
    let m: i32 = "10".parse().expect("Should be a valid integer");
    println!("expect: {}", m);

    // ----------------------------------------------------------
    // Result methods
    // ----------------------------------------------------------
    let x: Result<i32, &str> = Ok(5);
    let y: Result<i32, &str> = Err("oops");

    println!("is_ok: {}, is_err: {}", x.is_ok(), y.is_err());
    println!("unwrap_or: {}", y.unwrap_or(0));         // 0
    println!("map: {:?}", x.map(|n| n * 2));           // Ok(10)
    println!("ok: {:?}", y.ok());                       // None (converts Err to None)

    // ----------------------------------------------------------
    // The ? Operator — Error Propagation
    // ----------------------------------------------------------
    // `?` applied to a Result: if Ok, unwrap the value; if Err, return early.
    // It can only be used in functions that return Result (or Option).

    match parse_and_double("21") {
        Ok(result) => println!("parse_and_double(\"21\") = {}", result),
        Err(e)     => println!("Error: {}", e),
    }

    match parse_and_double("abc") {
        Ok(result) => println!("parse_and_double(\"abc\") = {}", result),
        Err(e)     => println!("Error: {}", e),
    }

    // ----------------------------------------------------------
    // Option<T> and error handling
    // ----------------------------------------------------------
    // Option can be converted to Result with ok_or / ok_or_else
    let names = vec!["Alice", "Bob"];
    match find_name(&names, "Bob") {
        Ok(name)  => println!("Found: {}", name),
        Err(e)    => println!("Error: {}", e),
    }
    match find_name(&names, "Dave") {
        Ok(name)  => println!("Found: {}", name),
        Err(e)    => println!("Error: {}", e),
    }

    // ----------------------------------------------------------
    // Custom Error Types
    // ----------------------------------------------------------
    let result = calculate_average(&[10, 20, 30]);
    println!("Average of [10,20,30]: {:?}", result); // Ok(20.0)

    let empty_result = calculate_average(&[]);
    println!("Average of []:         {:?}", empty_result); // Err(...)

    let div_result = safe_divide(10, 0);
    println!("10 / 0 = {:?}", div_result); // Err(DivisionByZero)

    // ----------------------------------------------------------
    // Chaining Results
    // ----------------------------------------------------------
    let chained = parse_and_double("5")
        .map(|n| n + 100)
        .map(|n| format!("Result: {}", n));
    println!("{:?}", chained); // Ok("Result: 110")

    println!("\n--- Tutorial 09 complete! ---");
}

// ----------------------------------------------------------
// Helper Functions
// ----------------------------------------------------------

/// Parses a string and doubles it, propagating errors with `?`.
fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()?; // if Err, return early with that error
    Ok(n * 2)
}

/// Looks up a name in a slice, returning an error if not found.
fn find_name<'a>(names: &[&'a str], target: &str) -> Result<&'a str, String> {
    names.iter()
        .find(|&&n| n == target)
        .copied()
        .ok_or_else(|| format!("'{}' not found", target))
}

// ----------------------------------------------------------
// Custom Error Types
// ----------------------------------------------------------

/// A custom error enum for math operations.
#[derive(Debug, PartialEq)]
enum MathError {
    DivisionByZero,
    EmptyInput,
}

// Implement Display so we can print our error with `{}`
impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "division by zero"),
            MathError::EmptyInput     => write!(f, "input must not be empty"),
        }
    }
}

fn safe_divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn calculate_average(values: &[i32]) -> Result<f64, MathError> {
    if values.is_empty() {
        return Err(MathError::EmptyInput);
    }
    let sum: i32 = values.iter().sum();
    Ok(sum as f64 / values.len() as f64)
}

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_double_ok() {
        assert_eq!(parse_and_double("5"), Ok(10));
    }

    #[test]
    fn test_parse_and_double_err() {
        assert!(parse_and_double("xyz").is_err());
    }

    #[test]
    fn test_find_name_found() {
        let names = vec!["Alice", "Bob"];
        assert_eq!(find_name(&names, "Alice"), Ok("Alice"));
    }

    #[test]
    fn test_find_name_not_found() {
        let names = vec!["Alice", "Bob"];
        assert!(find_name(&names, "Dave").is_err());
    }

    #[test]
    fn test_safe_divide_ok() {
        assert_eq!(safe_divide(10, 2), Ok(5));
    }

    #[test]
    fn test_safe_divide_by_zero() {
        assert_eq!(safe_divide(5, 0), Err(MathError::DivisionByZero));
    }

    #[test]
    fn test_calculate_average_ok() {
        let avg = calculate_average(&[1, 2, 3, 4, 5]).unwrap();
        assert!((avg - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_average_empty() {
        assert_eq!(calculate_average(&[]), Err(MathError::EmptyInput));
    }

    #[test]
    fn test_result_map() {
        let r: Result<i32, &str> = Ok(4);
        assert_eq!(r.map(|x| x * 2), Ok(8));
    }

    #[test]
    fn test_result_unwrap_or() {
        let r: Result<i32, &str> = Err("fail");
        assert_eq!(r.unwrap_or(99), 99);
    }
}
