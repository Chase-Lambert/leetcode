pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result: Vec<char> = Vec::new();
    let shortest_word_len = strs.iter().map(|w| w.len()).min().unwrap();

    for i in 0..shortest_word_len {
        let base_letter = strs[0].as_bytes()[i] as char;
        let other_letters = strs
            .iter()
            .skip(1)
            .map(|w| w.as_bytes()[i] as char)
            .collect::<Vec<_>>();

        if other_letters.iter().all(|c| *c == base_letter) {
            result.push(base_letter);
        } else {
            break;
        }
    }

    result.into_iter().collect()
}

#[test]
fn longest_common_prefix_ex_1() {
    let strs = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    assert_eq!("fl".to_string(), longest_common_prefix(strs));
}

#[test]
fn longest_common_prefix_ex_2() {
    let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    assert_eq!("".to_string(), longest_common_prefix(strs));
}
