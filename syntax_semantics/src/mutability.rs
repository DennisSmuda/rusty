/**
 * Mutability
 */
let x = 5; // Immutable by default
x = 6; // Error

// Changes binding - not the value
// - Points to another i32
let mut y = 5;
y = 6 // No Error


let mut x = 5;
let y = &mut x;
// y is an immutable binding to a mutable reference
// can't bind y to something else, but can change x (with '*y')

// Field Level Mutability
struct Point {
    x: i32,
    y: i32,
}

let mut a = Point { x: 5, y: 6 };
a.x = 10;

let b = Point { x: 5, y: 6};
b.x = 10; // Error: cannot assign to immutable field `b.x`.
