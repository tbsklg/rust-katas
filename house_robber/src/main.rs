use std::{cmp::max, collections::HashMap};

fn main() {
    rob(vec![1, 2, 3, 4]);
}

fn rob(nums: Vec<i32>) -> i32 {
    fn go(i: i32, nums: &Vec<i32>, memo: &mut HashMap<i32, i32>) -> i32 {
        if i == 0 {
            return nums[0];
        }

        if i == 1 {
            return max(nums[0], nums[1]);
        }

        if let Some(&v) = memo.get(&i) {
            return v;
        }

        let res = max(
            go(i - 2, nums, memo) + nums[i as usize],
            go(i - 1, nums, memo),
        );
        memo.insert(i, res);
        res
    }

    go(nums.len() as i32 - 1, &nums, &mut HashMap::new())
}

#[test]
fn example_1() {
    assert_eq!(4, rob(vec![1, 2, 3, 1]));
}

#[test]
fn example_2() {
    assert_eq!(12, rob(vec![2, 7, 9, 3, 1]));
}
