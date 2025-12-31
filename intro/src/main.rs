mod fib;

use crate::fib::fib;

fn main() {
    // even function
    let num = is_even(11);
    println!("is 11 a even function - {}", num);

    // fibonacci function
    let fib_val = fib(10);
    println!("10th fibonacci function is {}", fib_val);

    println!(
        "size of 'elephant' is {}",
        get_str_len(String::from("elephant"))
    );
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
