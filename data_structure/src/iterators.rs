pub fn increase_by_one(v: &mut Vec<i32>) -> Vec<i32> {
    // Using for loop
    // for i in v.iter_mut() {
    //     *i += 1;
    // }

    // Using while let loop
    let mut v_iter = v.iter_mut();
    while let Some(val) = v_iter.next() {
        *val += 1;
    }

    return v.to_vec();
}

pub fn find_sum(v: &Vec<i32>) -> i32 {
    let v_iter = v.iter();
    let sum = v_iter.sum();
    return sum;
}

pub fn filter_by_even_and_multiply_by_two(v: &Vec<i32>) -> Vec<i32> {
    let v_iter = v.iter();
    let result_vector = v_iter.filter(|x| *x % 2 == 0).map(|x| x * 2);
    return result_vector.collect();
}
