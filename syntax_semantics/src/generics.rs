/**
 * Generics
 * - parametric polymorphism
 */
enum Option<T> {
    Some(T),
    None,
}

let x: Option<i32> = Some(5);

// Example: Result (Generic over two types)
enum Result<T, E> {
    Ok(T),
    Err(E),
}

/** Generic Functions **/
fn takes_anything<T>(x: T) {
    // ...
}


/** Generic Structs **/
struct Point<T> {
    x: T,
    y: T,
}

let int_origin = Point { x: 0, y: 0 };
let float_origin = Point { x: 0.0, y: 0.0 };

// Add Implementations
impl<T> Point<T> {
    fn foo() {}
}
