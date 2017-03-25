/**
 * Patterns
 */
let x = 1;
// Pattern Matching against literals
match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"), // Any-Case
}

// Multiple Patterns
match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("everything else"),
}

// Destructuring inside pattern
match origin {
    Point { x, y } => println!("({},{})", x, y),
}
// Naming
match origin {
    Point { x: x1, y: y1 } => println!("({},{})", x1, y1),
}


/**
 * Ranges
 */
let x = 1;

match x {
    1 ... 5 => println!("one through five"),
    _ => println!("anything"),
}

let x = '💅';

match x {
    'a' ... 'j' => println!("early letter"),
    'k' ... 'z' => println!("late letter"),
    _ => println!("something else"),
}
