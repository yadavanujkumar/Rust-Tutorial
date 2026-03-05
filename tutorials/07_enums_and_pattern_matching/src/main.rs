// ============================================================
// Tutorial 07: Enums and Pattern Matching
// ============================================================
//
// Key concepts covered:
//   - Defining enums
//   - Enums with data (variants can hold values)
//   - The Option<T> enum (Rust's null-safety solution)
//   - The `match` expression
//   - Catch-all patterns (`_` and named catch-all)
//   - `if let` — concise single-pattern matching
//   - `while let` — looping on a pattern
//   - Nested pattern matching
//   - Enums with methods

fn main() {
    // ----------------------------------------------------------
    // Basic Enums
    // ----------------------------------------------------------
    let direction = Direction::North;
    println!("Moving: {:?}", direction);

    // Show all directions
    for dir in [Direction::North, Direction::South, Direction::East, Direction::West] {
        match dir {
            Direction::North => println!("Heading North"),
            Direction::South => println!("Heading South"),
            Direction::East  => println!("Heading East"),
            Direction::West  => println!("Heading West"),
        }
    }

    // ----------------------------------------------------------
    // Enums with Data
    // ----------------------------------------------------------
    // Each variant can hold different types and amounts of data.
    let msg1 = Message::Move { x: 10, y: 20 };
    let msg2 = Message::Write(String::from("hello"));
    let msg3 = Message::ChangeColor(255, 128, 0);
    let msg4 = Message::Quit;

    for msg in [msg1, msg2, msg3, msg4] {
        process_message(msg);
    }

    // ----------------------------------------------------------
    // Option<T>
    // ----------------------------------------------------------
    // Rust has NO null. Instead, it uses Option<T>:
    //   enum Option<T> {
    //       Some(T),
    //       None,
    //   }
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    println!("some_number = {:?}", some_number);
    println!("no_number   = {:?}", no_number);

    // You must handle both cases before using the value.
    match some_number {
        Some(n) => println!("Got a number: {}", n),
        None    => println!("No number found"),
    }

    // Useful Option methods:
    let doubled = some_number.map(|n| n * 2);
    println!("doubled = {:?}", doubled); // Some(84)

    let with_default = no_number.unwrap_or(0);
    println!("with_default = {}", with_default); // 0

    println!("is_some: {}, is_none: {}", some_number.is_some(), no_number.is_none());

    // ----------------------------------------------------------
    // match — exhaustive pattern matching
    // ----------------------------------------------------------
    let coin = Coin::Quarter;
    println!("Quarter value: {} cents", coin_value(&coin));
    println!("Penny value:   {} cents", coin_value(&Coin::Penny));
    println!("Nickel value:  {} cents", coin_value(&Coin::Nickel));
    println!("Dime value:    {} cents", coin_value(&Coin::Dime));

    // match with binding
    let roll = 7u32;
    match roll {
        1       => println!("One"),
        2 | 3   => println!("Two or Three"),         // multiple patterns
        4..=6   => println!("Four through Six"),     // range pattern
        n       => println!("Something else: {}", n), // bind catch-all
    }

    // match can return a value
    let description = match roll {
        1 => "one",
        2..=5 => "small",
        6..=9 => "medium",
        _ => "large", // `_` is the wildcard/catch-all (discards the value)
    };
    println!("roll {} is {}", roll, description);

    // ----------------------------------------------------------
    // if let — concise single-pattern matching
    // ----------------------------------------------------------
    let favorite: Option<&str> = Some("Rust");

    if let Some(lang) = favorite {
        println!("Favorite language: {}", lang);
    } else {
        println!("No favorite language set");
    }

    // ----------------------------------------------------------
    // while let
    // ----------------------------------------------------------
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();

    // ----------------------------------------------------------
    // Nested patterns and destructuring
    // ----------------------------------------------------------
    let point = (3, -2);
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On x-axis at {}", x),
        (0, y) => println!("On y-axis at {}", y),
        (x, y) if x > 0 && y > 0 => println!("Quadrant I: ({}, {})", x, y),
        (x, y) => println!("Other quadrant: ({}, {})", x, y),
    }

    // ----------------------------------------------------------
    // Enum methods
    // ----------------------------------------------------------
    let shape = Shape::Circle(5.0);
    println!("Area of {:?} = {:.4}", shape, shape.area());

    let rect_shape = Shape::Rectangle(4.0, 6.0);
    println!("Area of {:?} = {:.4}", rect_shape, rect_shape.area());

    let tri_shape = Shape::Triangle(3.0, 4.0, 5.0);
    println!("Area of {:?} = {:.4}", tri_shape, tri_shape.area());

    println!("\n--- Tutorial 07 complete! ---");
}

// ----------------------------------------------------------
// Enum Definitions
// ----------------------------------------------------------

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // struct variant
    Write(String),            // tuple variant
    ChangeColor(u8, u8, u8),  // tuple variant with multiple values
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit!"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text)   => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to rgb({}, {}, {})", r, g, b),
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn coin_value(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny   => 1,
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64), // three sides
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(a, b, c) => {
                // Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_value() {
        assert_eq!(coin_value(&Coin::Penny), 1);
        assert_eq!(coin_value(&Coin::Nickel), 5);
        assert_eq!(coin_value(&Coin::Dime), 10);
        assert_eq!(coin_value(&Coin::Quarter), 25);
    }

    #[test]
    fn test_option_map() {
        let x: Option<i32> = Some(5);
        assert_eq!(x.map(|n| n * 2), Some(10));
        let y: Option<i32> = None;
        assert_eq!(y.map(|n| n * 2), None);
    }

    #[test]
    fn test_option_unwrap_or() {
        let x: Option<i32> = None;
        assert_eq!(x.unwrap_or(42), 42);
    }

    #[test]
    fn test_shape_area() {
        let c = Shape::Circle(1.0);
        assert!((c.area() - std::f64::consts::PI).abs() < 1e-10);

        let r = Shape::Rectangle(3.0, 4.0);
        assert_eq!(r.area(), 12.0);

        // 3-4-5 right triangle: area = 6
        let t = Shape::Triangle(3.0, 4.0, 5.0);
        assert!((t.area() - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_if_let() {
        let val: Option<i32> = Some(99);
        let mut found = false;
        if let Some(_) = val {
            found = true;
        }
        assert!(found);
    }
}
