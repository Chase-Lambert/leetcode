pub fn find_max_consecutive_ones(nums: &[i32]) -> i32 {
    if !nums.contains(&1) {
        return 0;
    }

    let mut max = 0;
    let mut current = 1;
    for (idx, n) in nums.iter().skip(1).enumerate() {
        if *n == 1 && nums[idx] == 1 {
            current += 1;
            if current > max {
                max = current;
            }
        } else {
            current = 1;
        }
    }

    max
}

#[test]
fn max_consecutive_ones_example_1() {
    let nums = [1, 1, 0, 1, 1, 1];
    assert_eq!(3, find_max_consecutive_ones(&nums));
}

#[test]
fn max_consecutive_ones_example_2() {
    let nums = [1, 0, 1, 1, 0, 1];
    assert_eq!(2, find_max_consecutive_ones(&nums));
}
