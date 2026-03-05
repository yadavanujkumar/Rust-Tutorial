// ============================================================
// Tutorial 10: Traits and Generics
// ============================================================
//
// Key concepts covered:
//   - Defining and implementing traits
//   - Default trait implementations
//   - Trait objects (dynamic dispatch) with `dyn Trait`
//   - Generics: generic functions and structs
//   - Trait bounds: restricting generic types
//   - Multiple trait bounds with `+`
//   - The `where` clause for cleaner syntax
//   - Common standard library traits: Display, Debug, Clone, PartialOrd
//   - The `impl Trait` syntax (static dispatch)

use std::fmt;

fn main() {
    // ----------------------------------------------------------
    // Defining and Implementing Traits
    // ----------------------------------------------------------
    let dog = Dog { name: String::from("Buddy") };
    let cat = Cat { name: String::from("Whiskers") };

    dog.speak();
    cat.speak();
    println!("{}", dog.description());
    println!("{}", cat.description()); // uses default implementation

    // ----------------------------------------------------------
    // Trait as a parameter (impl Trait syntax — static dispatch)
    // ----------------------------------------------------------
    make_sound(&dog);
    make_sound(&cat);

    // ----------------------------------------------------------
    // Trait bounds
    // ----------------------------------------------------------
    let nums = vec![5, 2, 8, 1, 9];
    println!("Largest in {:?} = {}", nums, largest(&nums));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest in {:?} = {}", chars, largest(&chars));

    // ----------------------------------------------------------
    // Generic Structs
    // ----------------------------------------------------------
    let int_pair = Pair::new(5, 10);
    println!("Pair: ({}, {})", int_pair.first, int_pair.second);
    println!("Sum: {}", int_pair.sum());

    let str_pair = Pair::new("hello", "world");
    println!("Pair: ({}, {})", str_pair.first, str_pair.second);

    // ----------------------------------------------------------
    // Trait Objects (dynamic dispatch with `dyn Trait`)
    // ----------------------------------------------------------
    // When you need a collection of different types that share a trait,
    // use `Box<dyn Trait>` for heap-allocated trait objects.
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: String::from("Rex") }),
        Box::new(Cat { name: String::from("Felix") }),
        Box::new(Bird { name: String::from("Tweety") }),
    ];

    println!("\n--- All animals speak ---");
    for animal in &animals {
        animal.speak();
    }

    // ----------------------------------------------------------
    // Deriving Standard Traits
    // ----------------------------------------------------------
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1.clone();
    println!("p1={:?}, p2={:?}", p1, p2);
    println!("p1 == p2? {}", p1 == p2);
    println!("Distance from origin: {:.4}", p1.distance_from_origin());
    println!("Display: {}", p1);

    // ----------------------------------------------------------
    // Generic functions with where clause
    // ----------------------------------------------------------
    println!("compare_and_display 5 vs 10: {}", compare_and_display(5, 10));
    println!("compare_and_display 20 vs 3: {}", compare_and_display(20, 3));

    println!("\n--- Tutorial 10 complete! ---");
}

// ----------------------------------------------------------
// Trait Definitions
// ----------------------------------------------------------

trait Animal {
    /// Required method — implementors must provide this.
    fn speak(&self);

    /// Method with a default implementation.
    fn description(&self) -> String {
        format!("I am an animal that says: {}", self.sound())
    }

    /// Helper used by the default `description`. Implementors may override.
    fn sound(&self) -> &str {
        "..."
    }
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

struct Bird {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) {
        println!("{} says: Woof!", self.name);
    }
    fn sound(&self) -> &str { "Woof" }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("{} says: Meow!", self.name);
    }
    fn sound(&self) -> &str { "Meow" }
}

impl Animal for Bird {
    fn speak(&self) {
        println!("{} says: Tweet!", self.name);
    }
}

/// Uses `impl Trait` (static dispatch — compiler generates code per concrete type).
fn make_sound(animal: &impl Animal) {
    animal.speak();
}

// ----------------------------------------------------------
// Generic Functions with Trait Bounds
// ----------------------------------------------------------

/// Returns the largest element in a slice.
/// The `PartialOrd` bound allows `>` comparisons.
/// The `Copy` bound means we can assign `item` to `largest`.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// Compares two values and displays both with their order.
/// Using `where` clause for cleaner multi-bound syntax.
fn compare_and_display<T>(a: T, b: T) -> String
where
    T: fmt::Display + PartialOrd,
{
    if a >= b {
        format!("{} >= {}", a, b)
    } else {
        format!("{} < {}", a, b)
    }
}

// ----------------------------------------------------------
// Generic Structs
// ----------------------------------------------------------

struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Pair<T> {
        Pair { first, second }
    }
}

// Implement `sum` only when T supports addition and Copy.
impl Pair<i32> {
    fn sum(&self) -> i32 {
        self.first + self.second
    }
}

// ----------------------------------------------------------
// Structs implementing multiple standard traits
// ----------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_integers() {
        assert_eq!(largest(&[1, 5, 3, 2, 4]), 5);
    }

    #[test]
    fn test_largest_chars() {
        assert_eq!(largest(&['a', 'z', 'm']), 'z');
    }

    #[test]
    fn test_pair_sum() {
        let p = Pair::new(3, 7);
        assert_eq!(p.sum(), 10);
    }

    #[test]
    fn test_point_distance() {
        let p = Point { x: 3.0, y: 4.0 };
        assert!((p.distance_from_origin() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_point_display() {
        let p = Point { x: 1.0, y: 2.0 };
        assert_eq!(format!("{}", p), "(1, 2)");
    }

    #[test]
    fn test_point_clone_and_eq() {
        let p1 = Point { x: 1.0, y: 2.0 };
        let p2 = p1.clone();
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_compare_and_display() {
        assert_eq!(compare_and_display(5, 3), "5 >= 3");
        assert_eq!(compare_and_display(1, 9), "1 < 9");
    }

    #[test]
    fn test_trait_object() {
        let animals: Vec<Box<dyn Animal>> = vec![
            Box::new(Dog { name: String::from("Fido") }),
            Box::new(Cat { name: String::from("Kitty") }),
        ];
        // Just verify we can iterate and call methods without panicking.
        for a in &animals {
            let _ = a.description();
        }
    }
}
