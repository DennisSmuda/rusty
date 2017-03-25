use std::io;
/**
 * Safety & Speed through zero-cost-abstractions
 * -> Memory Safety
 *
 */

fn main() {
    println!("====================");
    println!("Syntax and Semantics");

    let a = 5; // Variable binding 5 to 'a'
    let b: u32 = 5 // Same thing without type inference
}
/**
 * Ownership
 *
 * Resources are 'owned by its scope'
 * Can ownership can be transfered
 * -> Borrowing & References
 */
fn foo() {
    // v(vector) memory allocated on stack
    // [1, 2, 3] memory allocated on heap
    let v = vec![1, 2, 3];
} // All resources cleared at end-of-scope

fn breaks() {
    let v = vec![1, 2, 3];
    let v2 = v; // Passing a reference with '&'
    println!("v[0] is {}", v[0]); // Will produce Error
                                  // Because v has been moved!
}


/**
 * References are passed by prepending '&'
 */
fn refPass(v: &Vec<i32>>) {
    v.push(5); // Can not borrow as mutable
}

let v = vec![];
refPass(&v);


/**
 * &mut References
 */
let mut x = 5;
{
    let y = &mut x;
    *y += 1;
}
println!("{}", x); // Will print 6

/**
 * Safety through scope rules
 *
 * - Borrowing can only happen 'to' a scope no greater than its origin scope
 * - Ownership must be given back before accessing
 */
fn main() {
    let mut x = 5;

    let y = &mut x;    // &mut borrow of `x` starts here.

    *y += 1;

    println!("{}", x); // Try to borrow `x` here.
} // &mut borrow of `x` ends here.

/**
 * Living Long Enough
 */
let y: &i32;
let x = 5;
y = &x;

println!("{}", y); // x does not live long enough error!
// y is declared before x -> y lives longer which is not allowed



/**
 * Lifetimes
 */
fn bar<'a>(x: &'a i32) // bar has one lifetime 'a
fn bar<'a, 'b>(...) // bar has two parameters with two lifetimes

