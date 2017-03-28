/**
 * Traits
 * - let compiler know about functionality a type must provide
 */
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea {
    fn area(&self) -> f64;
    fn is_larger(&self, &Self) -> bool;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn is_larger(&self, other: &Self) -> bool {
        self.area() > other.area()
    }
}

/** Trait Bounds **/
// Make Promises about a (generic) types behaviour
fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

/**
 * Must be in Scope
 * - only available if 'use'
 * - trait or type MUST be defined by you
 */
use std::fmt::Debug;
// Multiple trait bounds
fn foo<T: Clone + Debug>(x: T) {
    x.clone();
    println!("{:?}", x);
}

/** Where Clause **/
fn bar<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
    // More Legible, keeps signature closer to function name
}
