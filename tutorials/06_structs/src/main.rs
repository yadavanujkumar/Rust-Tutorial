// ============================================================
// Tutorial 06: Structs and Methods
// ============================================================
//
// Key concepts covered:
//   - Defining structs
//   - Creating instances and accessing fields
//   - Struct update syntax
//   - Tuple structs
//   - Unit-like structs
//   - The Debug trait with `#[derive(Debug)]`
//   - Methods with `impl` blocks
//   - Associated functions (like constructors)
//   - Multiple `impl` blocks

fn main() {
    // ----------------------------------------------------------
    // Defining and Instantiating Structs
    // ----------------------------------------------------------
    let alice = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        age: 30,
        active: true,
    };
    println!("User: {} ({})", alice.username, alice.email);

    // Fields are immutable by default when the instance is immutable.
    // To mutate, the whole instance must be `mut`.
    let mut bob = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        age: 25,
        active: true,
    };
    bob.email = String::from("bob.new@example.com");
    println!("Bob's new email: {}", bob.email);

    // ----------------------------------------------------------
    // Struct Update Syntax
    // ----------------------------------------------------------
    // Create a new instance from an existing one, changing some fields.
    let charlie = User {
        username: String::from("charlie"),
        email: String::from("charlie@example.com"),
        ..alice // remaining fields copied from alice
    };
    println!("Charlie: active={}, age={}", charlie.active, charlie.age);

    // ----------------------------------------------------------
    // Tuple Structs
    // ----------------------------------------------------------
    // Like tuples, but with a name. Fields accessed by index.
    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);
    println!("Black: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin: ({}, {})", origin.0, origin.1);

    // ----------------------------------------------------------
    // Unit-like Structs
    // ----------------------------------------------------------
    // A struct with no fields. Useful for implementing traits.
    let _marker = AlwaysEqual;

    // ----------------------------------------------------------
    // Debug formatting
    // ----------------------------------------------------------
    let rect = Rectangle::new(10, 5);
    println!("rect = {:?}", rect);   // short format
    println!("rect = {:#?}", rect);  // pretty format

    // ----------------------------------------------------------
    // Methods
    // ----------------------------------------------------------
    let area = rect.area();
    println!("Area of {:?} = {}", rect, area);
    println!("Perimeter = {}", rect.perimeter());
    println!("Is square? {}", rect.is_square());

    let big_rect = Rectangle::new(20, 20);
    println!("rect can hold big_rect? {}", rect.can_hold(&big_rect));
    println!("big_rect can hold rect? {}", big_rect.can_hold(&rect));

    // ----------------------------------------------------------
    // Associated Functions (like constructors)
    // ----------------------------------------------------------
    let square = Rectangle::square(8);
    println!("Square: {:?}", square);

    // ----------------------------------------------------------
    // A more complex struct example: Circle
    // ----------------------------------------------------------
    let c = Circle::new(5.0);
    println!("Circle with radius {:.1}: area={:.4}, circumference={:.4}",
        c.radius, c.area(), c.circumference());

    println!("\n--- Tutorial 06 complete! ---");
}

// ----------------------------------------------------------
// Struct Definitions
// ----------------------------------------------------------

struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

struct Color(u8, u8, u8);  // tuple struct
struct Point(f64, f64);     // tuple struct

struct AlwaysEqual; // unit-like struct

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// Associated function: creates a Rectangle (acts as a constructor).
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height } // shorthand when field and variable names match
    }

    /// Associated function: creates a square.
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    /// Method: returns the area of the rectangle.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Method: returns the perimeter.
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    /// Method: checks whether this is a square.
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// Method: checks whether this rectangle can hold another.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let r = Rectangle::new(4, 5);
        assert_eq!(r.area(), 20);
    }

    #[test]
    fn test_rectangle_perimeter() {
        let r = Rectangle::new(3, 7);
        assert_eq!(r.perimeter(), 20);
    }

    #[test]
    fn test_rectangle_is_square() {
        let sq = Rectangle::square(5);
        assert!(sq.is_square());
        let rect = Rectangle::new(3, 5);
        assert!(!rect.is_square());
    }

    #[test]
    fn test_can_hold() {
        let big = Rectangle::new(10, 10);
        let small = Rectangle::new(5, 5);
        assert!(big.can_hold(&small));
        assert!(!small.can_hold(&big));
    }

    #[test]
    fn test_circle_area() {
        let c = Circle::new(1.0);
        let expected = std::f64::consts::PI;
        assert!((c.area() - expected).abs() < 1e-10);
    }
}
