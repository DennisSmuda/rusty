/**
 * Method Syntax
 * - allows for foo.bar().baz()
 * - instead of baz(bar(foo))
 */
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // Method Implementation
    // take either self, &self or &mut self
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn main() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());
}

/**
 * Chaining method calls
 */
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    // Returns a new circle
    fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment }
    }
}

fn main() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());

    let d = c.grow(2.0).area(); // Chaining happens here
    println!("{}", d);
}
