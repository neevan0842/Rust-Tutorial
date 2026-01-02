mod utils;

use crate::utils::find_first_word;

fn main() {
    let my_string = String::from("Hello World");
    let word = find_first_word(&my_string);
    println!("The first word is: {}", word);

    // Demonstrating different string types
    let name = String::from("Rustacean"); // Create a String
    let string_slice = &name[0..4]; // Create a string slice
    let string_literal = "Hello, world!"; // String literal
    println!("{}, {}, {}", name, string_slice, string_literal);
}
