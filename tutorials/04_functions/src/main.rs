// ============================================================
// Tutorial 04: Functions
// ============================================================
//
// Key concepts covered:
//   - Defining and calling functions
//   - Parameters and type annotations
//   - Return values (explicit `return` vs. implicit last expression)
//   - Statements vs. expressions
//   - Function pointers
//   - Closures (anonymous functions / lambdas)
//   - Closures that capture their environment

fn main() {
    // ----------------------------------------------------------
    // Basic function calls
    // ----------------------------------------------------------
    greet("Alice");
    greet("Bob");

    let sum = add(3, 4);
    println!("3 + 4 = {}", sum);

    let squared = square(5);
    println!("5² = {}", squared);

    // ----------------------------------------------------------
    // Statements vs. Expressions
    // ----------------------------------------------------------
    // A *statement* performs an action and does NOT return a value.
    //   let x = 6;  ← this is a statement
    //
    // An *expression* evaluates to a value.
    //   { let x = 3; x + 1 }  ← the block evaluates to 4
    //
    // Functions return the value of their last expression
    // (no semicolon needed on the last line inside the function).

    let block_value = {
        let x = 3;
        x * x + 1 // no semicolon → this is an expression, returned from the block
    };
    println!("block_value = {}", block_value); // 10

    // ----------------------------------------------------------
    // Multiple return values via tuples
    // ----------------------------------------------------------
    let (min, max) = min_max(&[3, 1, 4, 1, 5, 9, 2, 6]);
    println!("min={}, max={}", min, max);

    // ----------------------------------------------------------
    // Early return with `return`
    // ----------------------------------------------------------
    println!("absolute(-7) = {}", absolute(-7));
    println!("absolute(3)  = {}", absolute(3));

    // ----------------------------------------------------------
    // Function Pointers
    // ----------------------------------------------------------
    // Functions can be passed around like values.
    let operation: fn(i32, i32) -> i32 = add;
    println!("Function pointer: add(10, 5) = {}", operation(10, 5));

    let ops: Vec<fn(i32, i32) -> i32> = vec![add, subtract, multiply];
    for op in &ops {
        println!("op(6, 2) = {}", op(6, 2));
    }

    // ----------------------------------------------------------
    // Closures
    // ----------------------------------------------------------
    // Closures are anonymous functions defined with `|params| body`.
    let double = |x: i32| x * 2;
    println!("double(5) = {}", double(5));

    // Closures can capture variables from their enclosing scope.
    let offset = 10;
    let add_offset = |x| x + offset; // captures `offset`
    println!("add_offset(5) = {}", add_offset(5)); // 15

    // Multi-line closures use a block body.
    let describe = |n: i32| {
        if n > 0 {
            "positive"
        } else if n < 0 {
            "negative"
        } else {
            "zero"
        }
    };
    println!("describe(-3) = {}", describe(-3));
    println!("describe(0)  = {}", describe(0));
    println!("describe(7)  = {}", describe(7));

    // Closures as arguments (higher-order functions)
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("Evens: {:?}", evens);

    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // Returning a closure from a function
    let multiplier = make_multiplier(3);
    println!("multiplier(7) = {}", multiplier(7)); // 21

    println!("\n--- Tutorial 04 complete! ---");
}

// ----------------------------------------------------------
// Function definitions
// ----------------------------------------------------------

/// Prints a greeting for the given name.
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

/// Returns the sum of two integers.
fn add(a: i32, b: i32) -> i32 {
    a + b // last expression (no semicolon) is the return value
}

/// Returns the difference of two integers.
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Returns the product of two integers.
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Returns the square of an integer.
fn square(n: i32) -> i32 {
    n * n
}

/// Returns the minimum and maximum values of a slice.
fn min_max(values: &[i32]) -> (i32, i32) {
    let mut min = values[0];
    let mut max = values[0];
    for &v in &values[1..] {
        if v < min { min = v; }
        if v > max { max = v; }
    }
    (min, max)
}

/// Returns the absolute value, demonstrating early `return`.
fn absolute(n: i32) -> i32 {
    if n < 0 {
        return -n; // early return
    }
    n // implicit return of last expression
}

/// Returns a closure that multiplies its argument by `factor`.
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor // `move` transfers ownership of `factor` into the closure
}

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(10, 4), 6);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 7), 21);
    }

    #[test]
    fn test_square() {
        assert_eq!(square(4), 16);
        assert_eq!(square(0), 0);
    }

    #[test]
    fn test_min_max() {
        let (min, max) = min_max(&[5, 3, 8, 1, 9]);
        assert_eq!(min, 1);
        assert_eq!(max, 9);
    }

    #[test]
    fn test_absolute() {
        assert_eq!(absolute(-5), 5);
        assert_eq!(absolute(3), 3);
        assert_eq!(absolute(0), 0);
    }

    #[test]
    fn test_make_multiplier() {
        let triple = make_multiplier(3);
        assert_eq!(triple(5), 15);
        assert_eq!(triple(0), 0);
    }

    #[test]
    fn test_closure_captures_env() {
        let base = 100;
        let add_base = |x| x + base;
        assert_eq!(add_base(5), 105);
    }
}
