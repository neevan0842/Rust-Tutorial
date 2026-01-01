use std::fs;

pub fn fib(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;
    if num == 0 {
        return first;
    }
    if num == 1 {
        return 1;
    }

    for _ in 0..num - 1 {
        let temp = second;
        second = second + first;
        first = temp;
    }

    return second;
}

pub fn find_first_a(st: String) -> Option<i32> {
    for (idx, char) in st.chars().enumerate() {
        if char == 'a' {
            return Some(idx as i32);
        }
    }
    return None;
}

pub fn read_from_file_custom(file_path: String) -> Result<String, String> {
    let content = fs::read_to_string(file_path);
    match content {
        Ok(file_content) => Ok(file_content),
        Err(_e) => Err(String::from("File not found")),
    }
}
