use std::collections::HashMap;

pub fn group_values_by_key(pairs: Vec<(String, i32)>) -> HashMap<String, Vec<i32>> {
    let mut map: HashMap<String, Vec<i32>> = HashMap::new();
    for (key, val) in pairs {
        if map.contains_key(&key) {
            map.get_mut(&key).unwrap().push(val);
        } else {
            map.insert(key, vec![val]);
        }
    }
    return map;
}
