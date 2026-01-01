fn main() {
    let mut s1 = String::from("hello");
    add_string(&mut s1); // mutable reference passed
    println!("{}", s1);
}

fn add_string(s2: &mut String) {
    // mutable reference received
    s2.push_str(" world");
}
