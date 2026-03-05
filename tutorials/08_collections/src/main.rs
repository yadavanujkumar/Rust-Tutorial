// ============================================================
// Tutorial 08: Collections — Vec, HashMap and HashSet
// ============================================================
//
// Key concepts covered:
//   - Vec<T>: growable, heap-allocated list
//   - HashMap<K, V>: key-value store
//   - HashSet<T>: unique values
//   - Common operations: push, pop, get, contains, remove, iterate
//   - Entry API for HashMap
//   - Sorting and searching

use std::collections::{HashMap, HashSet};

fn main() {
    // ----------------------------------------------------------
    // Vec<T> — Dynamic Array
    // ----------------------------------------------------------
    println!("=== Vec<T> ===");

    // Creating a Vec
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("After pushing: {:?}", v);

    // Using the vec! macro
    let v2 = vec![10, 20, 30, 40, 50];

    // Accessing elements
    let third = &v2[2]; // panics on out-of-bounds
    println!("Third element: {}", third);

    let maybe = v2.get(10); // returns Option<&T> — safe
    println!("Index 10: {:?}", maybe); // None

    // Iterating over a Vec
    print!("v2 elements: ");
    for x in &v2 {
        print!("{} ", x);
    }
    println!();

    // Mutable iteration
    let mut v3 = vec![1, 2, 3, 4, 5];
    for x in &mut v3 {
        *x *= 10;
    }
    println!("v3 after mutation: {:?}", v3);

    // pop removes and returns the last element
    let popped = v3.pop();
    println!("Popped: {:?}, v3 now: {:?}", popped, v3);

    // len, is_empty, contains
    println!("len={}, is_empty={}, contains 20? {}",
        v3.len(), v3.is_empty(), v3.contains(&20));

    // Slices of a Vec
    let slice = &v2[1..4];
    println!("Slice [1..4]: {:?}", slice);

    // Sorting
    let mut unsorted = vec![5, 2, 8, 1, 9, 3];
    unsorted.sort();
    println!("Sorted: {:?}", unsorted);
    unsorted.sort_by(|a, b| b.cmp(a)); // reverse
    println!("Reverse sorted: {:?}", unsorted);

    // Deduplication (must be sorted first)
    let mut with_dups = vec![1, 1, 2, 3, 3, 3, 4];
    with_dups.dedup();
    println!("Deduped: {:?}", with_dups);

    // Retaining only matching elements
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
    nums.retain(|&x| x % 2 == 0);
    println!("Even numbers retained: {:?}", nums);

    // ----------------------------------------------------------
    // HashMap<K, V>
    // ----------------------------------------------------------
    println!("\n=== HashMap<K, V> ===");

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"), 82);
    scores.insert(String::from("Charlie"), 91);

    println!("scores: {:?}", scores);

    // Accessing values
    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    }

    // Check existence
    println!("Has Dave? {}", scores.contains_key("Dave"));

    // Iterate over key-value pairs
    for (name, score) in &scores {
        println!("  {} → {}", name, score);
    }

    // Update: overwrite existing value
    scores.insert(String::from("Bob"), 88);
    println!("Bob's updated score: {:?}", scores.get("Bob"));

    // Entry API — insert only if key doesn't exist
    scores.entry(String::from("Dave")).or_insert(75);
    scores.entry(String::from("Alice")).or_insert(0); // won't overwrite
    println!("Dave: {:?}, Alice still: {:?}", scores.get("Dave"), scores.get("Alice"));

    // Entry API — modify existing value
    let alice_score = scores.entry(String::from("Alice")).or_insert(0);
    *alice_score += 5; // add bonus
    println!("Alice after bonus: {:?}", scores.get("Alice"));

    // Remove
    scores.remove("Charlie");
    println!("After removing Charlie: {:?}", scores.keys().collect::<Vec<_>>());

    // Word frequency count example
    let text = "hello world hello rust world hello";
    let mut freq: HashMap<&str, usize> = HashMap::new();
    for word in text.split_whitespace() {
        let count = freq.entry(word).or_insert(0);
        *count += 1;
    }
    println!("\nWord frequencies: {:?}", freq);

    // ----------------------------------------------------------
    // HashSet<T>
    // ----------------------------------------------------------
    println!("\n=== HashSet<T> ===");

    let mut set_a: HashSet<i32> = HashSet::new();
    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);
    set_a.insert(2); // duplicate — not inserted
    println!("set_a: {:?} (len={})", set_a, set_a.len()); // len = 3

    let set_b: HashSet<i32> = [2, 3, 4, 5].iter().cloned().collect();

    // Set operations
    let union: HashSet<_>        = set_a.union(&set_b).collect();
    let intersection: HashSet<_> = set_a.intersection(&set_b).collect();
    let difference: HashSet<_>   = set_a.difference(&set_b).collect(); // in a but not b
    let sym_diff: HashSet<_>     = set_a.symmetric_difference(&set_b).collect();

    println!("Union:                {:?}", union);
    println!("Intersection:         {:?}", intersection);
    println!("Difference (a - b):   {:?}", difference);
    println!("Symmetric difference: {:?}", sym_diff);

    println!("set_a contains 3? {}", set_a.contains(&3));
    println!("set_a is_subset of set_b? {}", set_a.is_subset(&set_b));

    println!("\n--- Tutorial 08 complete! ---");
}

// ============================================================
// Tests
// ============================================================
#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_vec_push_pop() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        assert_eq!(v.len(), 4);
        assert_eq!(v.pop(), Some(4));
        assert_eq!(v.len(), 3);
    }

    #[test]
    fn test_vec_contains() {
        let v = vec![10, 20, 30];
        assert!(v.contains(&20));
        assert!(!v.contains(&99));
    }

    #[test]
    fn test_vec_sort() {
        let mut v = vec![3, 1, 4, 1, 5, 9];
        v.sort();
        assert_eq!(v, vec![1, 1, 3, 4, 5, 9]);
    }

    #[test]
    fn test_vec_retain() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        v.retain(|&x| x % 2 == 0);
        assert_eq!(v, vec![2, 4, 6]);
    }

    #[test]
    fn test_hashmap_insert_get() {
        let mut map = HashMap::new();
        map.insert("key", 42);
        assert_eq!(map.get("key"), Some(&42));
        assert_eq!(map.get("missing"), None);
    }

    #[test]
    fn test_hashmap_entry_or_insert() {
        let mut map: HashMap<&str, i32> = HashMap::new();
        map.entry("x").or_insert(10);
        map.entry("x").or_insert(99); // should NOT overwrite
        assert_eq!(map["x"], 10);
    }

    #[test]
    fn test_hashset_no_duplicates() {
        let mut s: HashSet<i32> = HashSet::new();
        s.insert(1);
        s.insert(1);
        s.insert(2);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_hashset_intersection() {
        let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let b: HashSet<i32> = [2, 3, 4].iter().cloned().collect();
        let inter: HashSet<_> = a.intersection(&b).cloned().collect();
        assert!(inter.contains(&2));
        assert!(inter.contains(&3));
        assert!(!inter.contains(&1));
        assert_eq!(inter.len(), 2);
    }
}
