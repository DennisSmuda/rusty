
/**
 * match - replacement for stupid if/else-chains
 */
let x = 5;

// Each 'branch' has the form 'val => expression'
match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("Something false"),
}
