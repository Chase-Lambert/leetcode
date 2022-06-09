pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let x: Vec<u32> = x
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let rev_x: Vec<u32> = x.clone().into_iter().rev().collect();

    x == rev_x
}

#[test]
fn is_palindrome_example_1() {
    let x = 121;
    assert!(is_palindrome(x));
}

#[test]
fn is_palindrome_example_2() {
    let x = -121;
    assert!(!is_palindrome(x));
}
#[test]
fn is_palindrome_example_3() {
    let x = 10;
    assert!(!is_palindrome(x));
}
