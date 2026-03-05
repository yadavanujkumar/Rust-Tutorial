// ============================================================
// Tutorial 11: Iterators and Closures
// ============================================================
//
// Iterators and closures are central to writing idiomatic,
// expressive Rust code. They are zero-cost abstractions —
// just as fast as hand-written loops.
//
// Key concepts covered:
//   - The Iterator trait and `next()`
//   - Creating iterators: iter(), iter_mut(), into_iter()
//   - Consuming adaptors: sum, product, count, max, min, any, all, find
//   - Iterator adaptors (lazy): map, filter, take, skip, zip, enumerate, flat_map
//   - Collecting results with `collect()`
//   - Writing a custom iterator
//   - Closures that capture their environment
//   - move closures
//   - FnOnce, FnMut, Fn

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // ----------------------------------------------------------
    // Iterating basics
    // ----------------------------------------------------------
    // iter()      → borrows elements (&T)
    // iter_mut()  → mutably borrows elements (&mut T)
    // into_iter() → takes ownership of elements (T)

    print!("iter:  ");
    for n in numbers.iter() {
        print!("{} ", n);
    }
    println!();

    // ----------------------------------------------------------
    // Consuming adaptors (consume the iterator — can't reuse it)
    // ----------------------------------------------------------
    let sum: i32 = numbers.iter().sum();
    let product: i64 = vec![1i64, 2, 3, 4, 5].iter().product();
    let count = numbers.iter().count();
    let max = numbers.iter().max();
    let min = numbers.iter().min();

    println!("sum={}, product={}, count={}, max={:?}, min={:?}",
        sum, product, count, max, min);

    println!("any > 8?  {}", numbers.iter().any(|&x| x > 8));
    println!("all > 0?  {}", numbers.iter().all(|&x| x > 0));
    println!("find > 5: {:?}", numbers.iter().find(|&&x| x > 5));
    println!("position of 7: {:?}", numbers.iter().position(|&x| x == 7));

    // ----------------------------------------------------------
    // Iterator adaptors (lazy — only compute when consumed)
    // ----------------------------------------------------------

    // map: transform each element
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("doubled: {:?}", doubled);

    // filter: keep elements matching predicate
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("evens: {:?}", evens);

    // filter_map: map + filter combined (Some → keep, None → skip)
    let even_halves: Vec<i32> = numbers.iter()
        .filter_map(|&x| if x % 2 == 0 { Some(x / 2) } else { None })
        .collect();
    println!("even halves: {:?}", even_halves);

    // take / skip
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    let skip_five: Vec<&i32>   = numbers.iter().skip(5).collect();
    println!("take(3): {:?}", first_three);
    println!("skip(5): {:?}", skip_five);

    // enumerate: yields (index, &value)
    println!("Enumerated (first 3):");
    for (i, val) in numbers.iter().enumerate().take(3) {
        println!("  [{}] = {}", i, val);
    }

    // zip: pair two iterators together
    let letters = vec!['a', 'b', 'c'];
    let zipped: Vec<(i32, char)> = numbers.iter().copied().zip(letters.iter().copied()).collect();
    println!("zipped: {:?}", zipped);

    // chain: concatenate two iterators
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let chained: Vec<&i32> = a.iter().chain(b.iter()).collect();
    println!("chained: {:?}", chained);

    // flat_map: map then flatten one level
    let words = vec!["hello world", "foo bar"];
    let chars_split: Vec<&str> = words.iter().flat_map(|s| s.split_whitespace()).collect();
    println!("flat_map words: {:?}", chars_split);

    // flatten: flatten nested iterables
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    println!("flatten: {:?}", flat);

    // fold: accumulate with a starting value
    let sum_fold: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("fold sum: {}", sum_fold);

    // ----------------------------------------------------------
    // Method chaining (idiomatic Rust style)
    // ----------------------------------------------------------
    let result: Vec<String> = (1..=5)
        .filter(|x| x % 2 != 0)  // odd numbers: 1, 3, 5
        .map(|x| x * x)           // squares: 1, 9, 25
        .map(|x| format!("{}²", (x as f64).sqrt() as i32)) // label
        .collect();
    println!("Odd squares labeled: {:?}", result);

    // ----------------------------------------------------------
    // Ranges as iterators
    // ----------------------------------------------------------
    let sum_100: u32 = (1..=100).sum();
    println!("Sum 1..=100 = {}", sum_100); // 5050

    // ----------------------------------------------------------
    // Custom Iterator
    // ----------------------------------------------------------
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("First 10 Fibonacci: {:?}", fibs);

    // ----------------------------------------------------------
    // Closures in depth
    // ----------------------------------------------------------

    // Fn — borrows from environment immutably (can be called many times)
    let greeting = String::from("Hello");
    let say_hello = |name: &str| println!("{}, {}!", greeting, name);
    say_hello("Alice");
    say_hello("Bob");
    println!("greeting still usable: {}", greeting); // not moved

    // FnMut — mutably borrows from environment
    let mut total = 0;
    let mut accumulate = |x: i32| { total += x; };
    accumulate(5);
    accumulate(10);
    drop(accumulate); // drop borrow so we can use `total` again
    println!("total = {}", total); // 15

    // FnOnce — takes ownership (can only be called once)
    let name = String::from("Rust");
    let consume = move || println!("Consuming: {}", name); // `name` moved in
    consume();
    // consume(); // ERROR: can't call again — name was moved inside
    // println!("{}", name); // ERROR: moved into closure

    // ----------------------------------------------------------
    // Returning iterators
    // ----------------------------------------------------------
    let even_squares: Vec<u32> = even_squares_up_to(20).collect();
    println!("Even squares up to 20: {:?}", even_squares);

    println!("\n--- Tutorial 11 complete! ---");
}

// ----------------------------------------------------------
// Custom Iterator
// ----------------------------------------------------------

struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next_val = self.a;
        let new_b = self.a + self.b;
        self.a = self.b;
        self.b = new_b;
        Some(next_val) // infinite iterator — never returns None
    }
}

// ----------------------------------------------------------
// Returning an iterator with `impl Iterator`
// ----------------------------------------------------------

fn even_squares_up_to(max: u32) -> impl Iterator<Item = u32> {
    (1..=max)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
}

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let v = vec![1, 2, 3, 4, 5];
        let s: i32 = v.iter().sum();
        assert_eq!(s, 15);
    }

    #[test]
    fn test_map_filter() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let result: Vec<i32> = v.iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| x * x)
            .collect();
        assert_eq!(result, vec![4, 16, 36]);
    }

    #[test]
    fn test_any_all() {
        let v = vec![2, 4, 6, 8];
        assert!(v.iter().all(|&x| x % 2 == 0));
        assert!(v.iter().any(|&x| x > 5));
    }

    #[test]
    fn test_fold() {
        let v = vec![1, 2, 3, 4];
        let product = v.iter().fold(1, |acc, &x| acc * x);
        assert_eq!(product, 24);
    }

    #[test]
    fn test_fibonacci() {
        let fibs: Vec<u64> = Fibonacci::new().take(8).collect();
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn test_even_squares_up_to() {
        let result: Vec<u32> = even_squares_up_to(10).collect();
        // Even numbers ≤ 10: 2, 4, 6, 8, 10  →  squares: 4, 16, 36, 64, 100
        assert_eq!(result, vec![4, 16, 36, 64, 100]);
    }

    #[test]
    fn test_zip() {
        let a = vec![1, 2, 3];
        let b = vec!["one", "two", "three"];
        let zipped: Vec<(i32, &&str)> = a.iter().copied().zip(b.iter()).collect();
        assert_eq!(zipped[0], (1, &"one"));
        assert_eq!(zipped[2], (3, &"three"));
    }

    #[test]
    fn test_flatten() {
        let nested = vec![vec![1, 2], vec![3, 4]];
        let flat: Vec<i32> = nested.into_iter().flatten().collect();
        assert_eq!(flat, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_take_skip() {
        let v: Vec<i32> = (1..=10).collect();
        let taken: Vec<&i32> = v.iter().skip(3).take(4).collect();
        assert_eq!(taken, vec![&4, &5, &6, &7]);
    }

    #[test]
    fn test_closure_captures() {
        let factor = 3;
        let triple: Vec<i32> = vec![1, 2, 3].iter().map(|&x| x * factor).collect();
        assert_eq!(triple, vec![3, 6, 9]);
    }
}
