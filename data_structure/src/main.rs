mod hashmaps;
mod iterators;
mod vectors;

use crate::hashmaps::group_values_by_key;
use crate::iterators::{filter_by_even_and_multiply_by_two, find_sum, increase_by_one};
use crate::vectors::even_filter;

fn main() {
    // let mut v = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);
    // v.push(4);
    let v = vec![1, 2, 3, 4];

    let ans = even_filter(&v);
    println!("vector is {:?}", ans);
    println!("vector is {:?}", v);

    // map example
    let pairs: Vec<(String, i32)> = vec![
        (String::from('a'), 10),
        (String::from('b'), 20),
        (String::from('a'), 30),
    ];
    let map = group_values_by_key(pairs);
    println!("hashmap is {:?}", map);

    // iterator example
    let mut vec: Vec<i32> = vec![1, 2, 3, 4];
    increase_by_one(&mut vec);
    println!("increased vector is {:?}", vec);
    increase_by_one(&mut vec);
    println!("increased vector is {:?}", vec);

    // iterator types
    let v2 = vec![1, 2, 3];
    println!("sum is {}", find_sum(&v2));
    let iter2 = v2.iter().map(|x| x - 1);
    for i in iter2 {
        println!("value from iter1: {}", i);
    }
    let v3 = filter_by_even_and_multiply_by_two(&v2);
    println!("filtered vector is {:?}", v3);
}
