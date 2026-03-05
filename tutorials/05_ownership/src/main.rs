// ============================================================
// Tutorial 05: Ownership, Borrowing and Slices
// ============================================================
//
// Ownership is Rust's most unique feature and enables memory
// safety without a garbage collector.
//
// The three rules of ownership:
//   1. Each value has exactly ONE owner.
//   2. There can only be one owner at a time.
//   3. When the owner goes out of scope, the value is dropped.
//
// Key concepts covered:
//   - Ownership and move semantics
//   - Clone for deep copies
//   - Copy types (stack-only data)
//   - References and borrowing
//   - Mutable references
//   - The borrow checker rules
//   - Slices

fn main() {
    // ----------------------------------------------------------
    // Ownership and Move Semantics
    // ----------------------------------------------------------

    // String is allocated on the heap and is subject to ownership rules.
    let s1 = String::from("hello");
    // Assigning s1 to s2 MOVES ownership. s1 is no longer valid.
    let s2 = s1;
    // println!("{}", s1); // ERROR: value borrowed here after move
    println!("s2 owns the string: {}", s2);

    // ----------------------------------------------------------
    // Clone — deep copy
    // ----------------------------------------------------------
    let s3 = String::from("world");
    let s4 = s3.clone(); // heap data is copied
    println!("s3={}, s4={}", s3, s4); // both are valid

    // ----------------------------------------------------------
    // Copy types (stack-only)
    // ----------------------------------------------------------
    // Types like integers, floats, booleans, chars implement `Copy`.
    // Assigning them copies the value; the original is still valid.
    let x = 5;
    let y = x; // copy, not move
    println!("x={}, y={}", x, y); // both valid

    // ----------------------------------------------------------
    // Ownership and Functions
    // ----------------------------------------------------------
    let s = String::from("ownership");
    takes_ownership(s); // s is moved into the function
    // println!("{}", s); // ERROR: s is no longer valid here

    let n = 42;
    makes_copy(n); // i32 is Copy, so n is still valid
    println!("n is still valid: {}", n);

    // Getting ownership back via return value
    let s5 = String::from("returned");
    let s6 = takes_and_gives_back(s5);
    println!("s6 = {}", s6);

    // ----------------------------------------------------------
    // References and Borrowing
    // ----------------------------------------------------------
    // A reference allows you to refer to a value WITHOUT taking ownership.
    // References are denoted with `&`.
    let s7 = String::from("hello");
    let len = calculate_length(&s7); // borrow s7
    println!("The length of '{}' is {}", s7, len); // s7 still valid

    // ----------------------------------------------------------
    // Mutable References
    // ----------------------------------------------------------
    let mut s8 = String::from("hello");
    change(&mut s8); // mutable borrow
    println!("After change: {}", s8);

    // RULE: You can have EITHER one mutable reference OR any number of
    // immutable references to a value in a given scope — but not both.
    let mut s9 = String::from("hello");
    let r1 = &s9;     // immutable borrow
    let r2 = &s9;     // another immutable borrow — OK
    println!("r1={}, r2={}", r1, r2);
    // r1 and r2 are no longer used after this point, so the mutable borrow below is fine.
    let r3 = &mut s9; // mutable borrow — OK because r1, r2 are no longer used
    r3.push_str(", world");
    println!("r3={}", r3);

    // ----------------------------------------------------------
    // Dangling References
    // ----------------------------------------------------------
    // Rust prevents dangling references at compile time.
    // The function below is commented out because it would not compile:
    // let reference = dangle(); // ERROR: returns reference to local data

    // ----------------------------------------------------------
    // Slices
    // ----------------------------------------------------------
    // A slice is a reference to a contiguous sequence of elements.
    // Slices do NOT have ownership.

    // String slices
    let sentence = String::from("hello world");
    let hello = &sentence[0..5];   // "hello"
    let world = &sentence[6..11];  // "world"
    println!("Slice: '{}' '{}'", hello, world);

    // First word demo
    let first = first_word(&sentence);
    println!("First word: '{}'", first);

    // &str literals ARE string slices.
    let literal: &str = "I am a string slice";
    println!("Literal slice: {}", literal);

    // Array slices
    let arr = [1, 2, 3, 4, 5];
    let arr_slice: &[i32] = &arr[1..4]; // [2, 3, 4]
    println!("Array slice: {:?}", arr_slice);

    println!("\n--- Tutorial 05 complete! ---");
}

// ----------------------------------------------------------
// Helper functions demonstrating ownership rules
// ----------------------------------------------------------

fn takes_ownership(s: String) {
    println!("takes_ownership got: {}", s);
} // s is dropped here

fn makes_copy(n: i32) {
    println!("makes_copy got: {}", n);
}

fn takes_and_gives_back(s: String) -> String {
    s // returned, ownership moves to caller
}

fn calculate_length(s: &String) -> usize {
    s.len() // s is borrowed, not owned; it is NOT dropped when this function ends
}

fn change(s: &mut String) {
    s.push_str(", world");
}

/// Returns the first word in a string slice (up to first space).
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    &s[..] // whole string is one word
}

// The following would NOT compile — it would create a dangling reference:
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // ERROR: s is dropped at end of this function, but we return a ref to it
// }

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let s1 = String::from("clone me");
        let s2 = s1.clone();
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        assert_eq!(calculate_length(&s), 5);
    }

    #[test]
    fn test_change_mutable_ref() {
        let mut s = String::from("hello");
        change(&mut s);
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("singleword"), "singleword");
        assert_eq!(first_word("leading space"), "leading");
    }

    #[test]
    fn test_array_slice() {
        let arr = [10, 20, 30, 40, 50];
        let slice = &arr[1..4];
        assert_eq!(slice, &[20, 30, 40]);
    }
}
