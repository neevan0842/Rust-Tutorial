pub fn find_first_word(str: &String) -> &str {
    let mut index = 0;
    for i in str.chars() {
        if i == ' ' {
            break;
        }
        index += 1;
    }
    return &str[0..index];
}
