pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut solution = Vec::new();

    for (idxn, n) in nums.iter().enumerate() {
        for (idxm, m) in nums.iter().enumerate() {
            if m + n == target && idxm != idxn {
                solution.push(idxn as i32);
                solution.push(idxm as i32);
            }
        }
    }

    solution.sort_unstable();
    solution.dedup();
    solution
}

#[test]
fn two_sum_example_1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(vec![0, 1], two_sum(nums, target));
}

#[test]
fn two_sum_example_2() {
    let nums = vec![3, 2, 4];
    let target = 6;
    assert_eq!(vec![1, 2], two_sum(nums, target));
}
#[test]
fn two_sum_example_3() {
    let nums = vec![3, 3];
    let target = 6;
    assert_eq!(vec![0, 1], two_sum(nums, target));
}
