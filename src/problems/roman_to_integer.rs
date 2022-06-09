pub fn roman_to_int(s: String) -> i32 {
    let symbols: Vec<char> = s.chars().rev().collect();
    let mut result = get_val(symbols[0]);

    for (idx, c) in symbols.iter().skip(1).enumerate() {
        match (symbols[idx + 1], symbols[idx]) {
            ('I', 'V') | ('I', 'X') => {
                result -= 1;
            }
            ('X', 'L') | ('X', 'C') => {
                result -= 10;
            }
            ('C', 'D') | ('C', 'M') => {
                result -= 100;
            }
            _ => result += get_val(*c),
        }
    }

    result
}

fn get_val(symbol: char) -> i32 {
    match symbol {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => unreachable!(),
    }
}

#[test]
fn roman_to_int_example_1() {
    let s = "III".to_string();
    assert_eq!(3, roman_to_int(s));
}

#[test]
fn roman_to_int_example_2() {
    let s = "LVIII".to_string();
    assert_eq!(58, roman_to_int(s));
}

#[test]
fn roman_to_int_example_3() {
    let s = "MCMXCIV".to_string();
    assert_eq!(1994, roman_to_int(s));
}
