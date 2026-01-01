pub fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_v = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_v.push(*val);
        }
    }
    return new_v;
}
