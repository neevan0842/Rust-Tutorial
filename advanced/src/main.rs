mod multithreading;

use crate::multithreading::{
    demonstrate_move_ownership, run_multithreading_example, sum_large_range,
};
use num_format::{Locale, ToFormattedString};

fn main() {
    // Demonstrate the use of a generic function
    let x = 10;
    let y = 20;
    let a = "apple";
    let b = "banana";
    println!("The largest number is: {}", find_largest(x, y));
    println!("The largest string is: {}", find_largest(a, b));

    // Demonstrate the use of a trait with a default implementation
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };
    notify(&user);

    // Run the multithreading example
    run_multithreading_example();
    demonstrate_move_ownership();
    let sum = sum_large_range();
    println!(
        "The sum of numbers from 0 to 10^10 is: {}",
        sum.to_formatted_string(&Locale::en)
    );
}

// Generic function to find the largest of two values
fn find_largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

trait Summary {
    // Default implementation of summarize
    fn summarize(&self) -> String {
        return String::from("Default implementation of summarize");
    }
}

struct User {
    name: String,
    age: i16,
}

impl Summary for User {
    // Custom implementation of summarize for User
    fn summarize(&self) -> String {
        format!("User: {}, Age: {}", self.name, self.age)
    }
}

// Function that takes a reference to any type that implements Summary
fn notify(item: &impl Summary) {
    println!("Notification: {}", item.summarize());
}
