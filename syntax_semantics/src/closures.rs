/**
 * Closures
 * - syntax sugar for 'inline traits'
 */
let plus_two = |x| {
    let mut result: i32 = x;

    result += 1;
    result += 1;

    result
};

assert_eq!(4, plus_two(2));


/** Move Closures **/
let num = 5;
// Force closure to take ownership of its environment
let owns_num = move |x: i32| x + num;

/** As Arguments **/
fn call_with_one<F>(some_closure: F) -> i32
    where F: Fn(i32) -> i32 {

    some_closure(1)
}

let answer = call_with_one(|x| x + 2);

assert_eq!(3, answer);
