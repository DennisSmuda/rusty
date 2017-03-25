
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    println!("The origin is at ({}, {})", origin.x, origin.y);
}


/**
 * Tuple Structs
 */
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// Access through dot-notation or destructuring
let black_r = black.0;
let Point(_, origin_y, origin_z) = origin;
