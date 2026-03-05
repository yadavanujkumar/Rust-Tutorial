// ============================================================
// Tutorial 01: Hello World
// ============================================================
//
// This is your first Rust program. Every Rust program starts
// with a `main` function, which is the entry point.
//
// Key concepts covered:
//   - The `main` function
//   - Printing to the console with `println!`
//   - Rust macros (identifiable by the `!` suffix)
//   - Single-line and multi-line comments
//   - Basic string formatting with `{}`

fn main() {
    // println! is a macro (not a function) that prints a line to stdout.
    println!("Hello, World!");

    // You can print variables using `{}` as a placeholder.
    let name = "Rustacean";
    println!("Hello, {}!", name);

    // Multiple placeholders are filled left-to-right.
    let language = "Rust";
    let version = 1;
    println!("Welcome to {} Tutorial, chapter {}!", language, version);

    // Use `{:?}` (debug format) to print types that implement the Debug trait.
    let numbers = [1, 2, 3];
    println!("Numbers: {:?}", numbers);

    // `{:#?}` gives a pretty-printed debug output.
    println!("Numbers (pretty): {:#?}", numbers);

    // `print!` works like `println!` but does NOT add a newline.
    print!("No newline here... ");
    println!("but here there is one.");

    // eprintln! prints to stderr instead of stdout.
    eprintln!("This goes to standard error.");

    println!("\n--- Tutorial 01 complete! ---");
}

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    // A simple sanity test to make sure this module compiles.
    #[test]
    fn test_hello() {
        let greeting = format!("Hello, {}!", "World");
        assert_eq!(greeting, "Hello, World!");
    }

    #[test]
    fn test_format_with_multiple_args() {
        let result = format!("{} + {} = {}", 1, 2, 3);
        assert_eq!(result, "1 + 2 = 3");
    }
}
