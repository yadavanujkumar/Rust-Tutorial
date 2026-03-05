// ============================================================
// Tutorial 03: Control Flow
// ============================================================
//
// Key concepts covered:
//   - `if` / `else if` / `else` expressions
//   - `if` as an expression (returning a value)
//   - `loop` — infinite loop with `break`
//   - Returning values from `loop`
//   - `while` loop
//   - `for` loop and ranges
//   - Nested loops and loop labels

fn main() {
    // ----------------------------------------------------------
    // if / else if / else
    // ----------------------------------------------------------
    let number = 7;

    if number < 5 {
        println!("{} is less than 5", number);
    } else if number == 5 {
        println!("{} is equal to 5", number);
    } else {
        println!("{} is greater than 5", number);
    }

    // Conditions must be `bool`; Rust does NOT auto-convert integers.
    // if number { ... }  // ERROR: expected bool, found integer

    // ----------------------------------------------------------
    // if as an expression
    // ----------------------------------------------------------
    // In Rust, `if` is an expression and can return a value.
    // Both branches must produce the same type.
    let condition = true;
    let value = if condition { 10 } else { 20 };
    println!("value = {}", value); // 10

    // ----------------------------------------------------------
    // loop — infinite loop
    // ----------------------------------------------------------
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 3 {
            break;
        }
    }
    println!("loop stopped at counter = {}", counter);

    // Returning a value from `loop` with `break expr`
    let mut n = 0;
    let result = loop {
        n += 1;
        if n == 5 {
            break n * 2; // returns 10
        }
    };
    println!("loop result = {}", result); // 10

    // ----------------------------------------------------------
    // while loop
    // ----------------------------------------------------------
    let mut count = 3;
    while count > 0 {
        println!("while countdown: {}", count);
        count -= 1;
    }
    println!("Blast off!");

    // ----------------------------------------------------------
    // for loop over a collection
    // ----------------------------------------------------------
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        print!("{} ", element);
    }
    println!();

    // Iterate with index using `enumerate()`
    for (i, val) in arr.iter().enumerate() {
        println!("arr[{}] = {}", i, val);
    }

    // ----------------------------------------------------------
    // Ranges
    // ----------------------------------------------------------
    // `1..5`  → exclusive end: 1, 2, 3, 4
    // `1..=5` → inclusive end: 1, 2, 3, 4, 5
    print!("Exclusive range (1..5): ");
    for i in 1..5 {
        print!("{} ", i);
    }
    println!();

    print!("Inclusive range (1..=5): ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    // Reverse a range with `.rev()`
    print!("Reverse (5..=1 via rev): ");
    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!();

    // ----------------------------------------------------------
    // Loop labels (for nested loops)
    // ----------------------------------------------------------
    'outer: for x in 0..3 {
        for y in 0..3 {
            if x == 1 && y == 1 {
                println!("Breaking outer loop at x={}, y={}", x, y);
                break 'outer; // breaks the outer loop
            }
            println!("  x={}, y={}", x, y);
        }
    }

    // ----------------------------------------------------------
    // continue
    // ----------------------------------------------------------
    print!("Odd numbers 1-10: ");
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // skip even numbers
        }
        print!("{} ", i);
    }
    println!();

    println!("\n--- Tutorial 03 complete! ---");
}

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    #[test]
    fn test_if_expression() {
        let x = 3;
        let result = if x > 2 { "big" } else { "small" };
        assert_eq!(result, "big");
    }

    #[test]
    fn test_loop_with_break_value() {
        let mut i = 0;
        let val = loop {
            i += 1;
            if i == 4 {
                break i * 10;
            }
        };
        assert_eq!(val, 40);
    }

    #[test]
    fn test_while_loop() {
        let mut sum = 0;
        let mut n = 1;
        while n <= 5 {
            sum += n;
            n += 1;
        }
        assert_eq!(sum, 15); // 1+2+3+4+5
    }

    #[test]
    fn test_for_range() {
        let mut product = 1;
        for i in 1..=4 {
            product *= i;
        }
        assert_eq!(product, 24); // 1*2*3*4
    }

    #[test]
    fn test_for_collection() {
        let nums = [1, 2, 3, 4, 5];
        let mut sum = 0;
        for n in nums {
            sum += n;
        }
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_continue_skips() {
        let mut evens = vec![];
        for i in 0..10 {
            if i % 2 != 0 {
                continue;
            }
            evens.push(i);
        }
        assert_eq!(evens, vec![0, 2, 4, 6, 8]);
    }
}
