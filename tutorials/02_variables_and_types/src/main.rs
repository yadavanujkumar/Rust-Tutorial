// ============================================================
// Tutorial 02: Variables, Mutability and Data Types
// ============================================================
//
// Key concepts covered:
//   - Immutable variables (default) with `let`
//   - Mutable variables with `let mut`
//   - Variable shadowing
//   - Constants with `const`
//   - Scalar types: integers, floats, booleans, chars
//   - Compound types: tuples and arrays
//   - Type annotations and type inference

fn main() {
    // ----------------------------------------------------------
    // Variables and Mutability
    // ----------------------------------------------------------

    // By default, variables are IMMUTABLE in Rust.
    let x = 5;
    println!("Immutable x = {}", x);

    // Attempting to assign to `x` again would be a compile error:
    // x = 6; // ERROR: cannot assign twice to immutable variable

    // Use `mut` to make a variable mutable.
    let mut y = 10;
    println!("Mutable y before = {}", y);
    y = 20;
    println!("Mutable y after  = {}", y);

    // ----------------------------------------------------------
    // Shadowing
    // ----------------------------------------------------------
    // You can re-declare a variable with the same name, "shadowing" it.
    // Shadowing allows you to change the type, unlike `mut`.
    let spaces = "   "; // &str
    let spaces = spaces.len(); // usize — same name, different type
    println!("spaces (length) = {}", spaces);

    let shadow = 5;
    let shadow = shadow + 1; // shadow = 6
    {
        let shadow = shadow * 2; // shadow = 12 (only inside this block)
        println!("Inner shadow = {}", shadow);
    }
    println!("Outer shadow = {}", shadow); // back to 6

    // ----------------------------------------------------------
    // Constants
    // ----------------------------------------------------------
    // Constants are always immutable and must have a type annotation.
    // They can be declared in any scope, including global scope.
    const MAX_POINTS: u32 = 100_000; // underscores improve readability
    println!("MAX_POINTS = {}", MAX_POINTS);

    // ----------------------------------------------------------
    // Integer Types
    // ----------------------------------------------------------
    // Signed:   i8, i16, i32 (default), i64, i128, isize
    // Unsigned: u8, u16, u32, u64, u128, usize
    let a: i8 = -128;
    let b: u8 = 255;
    let c: i32 = 2_147_483_647;
    let d: u64 = 18_446_744_073_709_551_615;
    println!("i8 min: {}, u8 max: {}, i32 max: {}, u64 max: {}", a, b, c, d);

    // Integer literals can be written in different bases:
    let decimal = 98_222;
    let hex = 0xff;       // 255
    let octal = 0o77;     // 63
    let binary = 0b1111_0000; // 240
    let byte: u8 = b'A';  // 65
    println!("decimal={} hex={} octal={} binary={} byte={}", decimal, hex, octal, binary, byte);

    // ----------------------------------------------------------
    // Floating-Point Types
    // ----------------------------------------------------------
    // f32: single precision, f64 (default): double precision
    let f1: f32 = 3.14;
    let f2: f64 = 2.718_281_828;
    println!("f32: {}, f64: {}", f1, f2);

    // Arithmetic operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("sum={} diff={:.2} product={} quotient={:.4} remainder={}", sum, difference, product, quotient, remainder);

    // ----------------------------------------------------------
    // Boolean Type
    // ----------------------------------------------------------
    let t: bool = true;
    let f: bool = false;
    println!("true AND false = {}", t && f);
    println!("true OR  false = {}", t || f);
    println!("NOT true       = {}", !t);

    // ----------------------------------------------------------
    // Character Type
    // ----------------------------------------------------------
    // Rust's `char` is a Unicode scalar value (4 bytes).
    // char literals use single quotes.
    let letter: char = 'z';
    let emoji: char = '😻';
    let chinese: char = '中';
    println!("letter={}, emoji={}, chinese={}", letter, emoji, chinese);

    // ----------------------------------------------------------
    // Tuples
    // ----------------------------------------------------------
    // A tuple groups values of different types. Fixed length.
    let tup: (i32, f64, bool, char) = (500, 6.4, true, 'y');

    // Destructuring a tuple
    let (p, q, r, s) = tup;
    println!("Destructured: p={} q={} r={} s={}", p, q, r, s);

    // Access by index using dot notation
    println!("tup.0={} tup.1={} tup.2={} tup.3={}", tup.0, tup.1, tup.2, tup.3);

    // The unit type `()` is an empty tuple — represents "nothing".
    let unit: () = ();
    println!("Unit value: {:?}", unit);

    // ----------------------------------------------------------
    // Arrays
    // ----------------------------------------------------------
    // Arrays have a fixed length and all elements must be the same type.
    // Stored on the stack.
    let arr = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);
    println!("First element: {}", arr[0]);
    println!("Array length: {}", arr.len());

    // Type annotation: [type; length]
    let typed_arr: [i32; 3] = [10, 20, 30];
    println!("Typed array: {:?}", typed_arr);

    // Initialize all elements to the same value: [value; count]
    let zeros = [0; 5];
    println!("Zeros: {:?}", zeros);

    println!("\n--- Tutorial 02 complete! ---");
}

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    #[test]
    fn test_mutability() {
        let mut x = 5;
        x += 5;
        assert_eq!(x, 10);
    }

    #[test]
    fn test_shadowing() {
        let x = "hello";
        let x = x.len();
        assert_eq!(x, 5);
    }

    #[test]
    fn test_tuple_access() {
        let tup = (1, 2.0, 'a');
        assert_eq!(tup.0, 1);
        assert_eq!(tup.1, 2.0_f64);
        assert_eq!(tup.2, 'a');
    }

    #[test]
    fn test_array_access() {
        let arr = [10, 20, 30];
        assert_eq!(arr[0], 10);
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_integer_arithmetic() {
        assert_eq!(2 + 3, 5);
        assert_eq!(10 - 4, 6);
        assert_eq!(3 * 7, 21);
        assert_eq!(10 / 3, 3); // integer division truncates
        assert_eq!(10 % 3, 1);
    }

    #[test]
    fn test_boolean_ops() {
        assert!(true && true);
        assert!(true || false);
        assert!(!false);
    }
}
