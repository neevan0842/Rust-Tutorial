mod rect;
mod utils;

use crate::rect::Rect;
use crate::utils::{fib, find_first_a, read_from_file_custom};
use chrono::Local;

fn main() {
    // even function
    let num = is_even(11);
    println!("is 11 a even function - {}", num);

    // fibonacci function
    let fib_val = fib(10);
    println!("10th fibonacci function is {}", fib_val);

    // string size
    println!(
        "size of 'elephant' is {}",
        get_str_len(String::from("elephant"))
    );

    let rectangle = Rect {
        length: 20,
        breath: 10,
    };
    // structure
    println!(
        "area is {} and perimeter is {}",
        rectangle.area(),
        rectangle.perimeter()
    );

    //enum
    let shape = Shape::Circle;
    let shape_with_value = ShapeWithValues::Circle(20.0);
    println!("shape is {:?}", shape);
    println!(
        "area of shape is {:?} is {:.2}",
        shape_with_value,
        shape_with_value.area()
    );

    // option enum
    let index = find_first_a(String::from("harkirat"));
    match index {
        Some(value) => println!("first a in 'harkirat' is at position {}", value),
        None => println!("'a' not found in string"),
    }

    //result enum
    let file_content = read_from_file_custom(String::from("some_file.txt"));
    match file_content {
        Ok(content) => println!("file content is {}", content),
        Err(e) => println!("error occurred: {}", e),
    }

    //using external crate
    let local_time = Local::now();
    println!("local time is {}", local_time);
}

fn get_str_len(s: String) -> usize {
    s.chars().count()
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Shape {
    Circle,
    Square,
}

#[derive(Debug)]
#[allow(dead_code)]
enum ShapeWithValues {
    Circle(f64),
    Rectangle(f64, f64),
}

impl ShapeWithValues {
    fn area(&self) -> f64 {
        // implicitily returns the value
        match self {
            ShapeWithValues::Rectangle(l, b) => l * b,
            ShapeWithValues::Circle(r) => std::f64::consts::PI * r * r,
        }
    }
}
