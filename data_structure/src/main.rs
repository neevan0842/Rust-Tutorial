mod hashmaps;
mod vectors;

use crate::hashmaps::group_values_by_key;
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
    let pairs = vec![
        (String::from('a'), 10),
        (String::from('b'), 20),
        (String::from('a'), 30),
    ];
    let map = group_values_by_key(pairs);
    println!("hashmap is {:?}", map);
}
