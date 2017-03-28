/**
 * if let
 * - syntax sugar for certain pattern matches
 */
if let Some(x) = option {
    foo(x);
}

/**
 * while let
 * - loops
 */
let mut v = vec![1, 3, 5, 7, 11];
while let Some(x) = v.pop() {
    println!("{}", x);
}
