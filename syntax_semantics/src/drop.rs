/**
 * Drop
 * - Trait in rust stl
 */
struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn main() {
    let x = HasDrop;

    // Do stuff.

} // `x` goes out of scope here.
