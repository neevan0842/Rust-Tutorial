mod vectors;

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

    // cannot do this because ownership of v is given to even_filter
    println!("vector is {:?}", v);
}
