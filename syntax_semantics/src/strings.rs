/**
 * Strings
 * - UTF-8 sequences
 *
 * &str and String
 */
let greeting = "Hello World!"; // greeting: &'static str
// String literal - string slice living for entire programm duration


/** String **/
let mut s = "Hello".to_string(); // mut s: String
println!("{}", s);

s.push_str(", world.");
println!("{}", s);


/** Slicing **/
let dog = "hachiko";
let hachi = &dog[0..5];

/** Concatenation **/
let hello = "Hello ".to_string();
let world = "world!";
// String + &str
let hello_world = hello + world;

let hello = "Hello ".to_string();
let world = "world!".to_string();
// Deref secend string - coerce to a &str
let hello_world = hello + &world;
