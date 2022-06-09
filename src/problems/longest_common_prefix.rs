pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result: Vec<char> = Vec::new();

    result.into_iter().collect()
}

#[test]
fn longest_common_prefix_example_1() {
    let strs = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    assert_eq!("fl".to_string(), longest_common_prefix(strs));
}

#[test]
fn longest_common_prefix_example_2() {
    let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    assert_eq!("".to_string(), longest_common_prefix(strs));
}
