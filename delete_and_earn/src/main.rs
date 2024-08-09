use std::{cmp::max, collections::HashMap};

fn main() {
    delete_and_earn(vec![2, 2, 3, 3, 3, 4]);
}

fn delete_and_earn(nums: Vec<i32>) -> i32 {
    fn go(i: i32, memo: &mut HashMap<i32, i32>, points: &HashMap<i32, i32>) -> i32 {
        if i == 0 {
            return 0;
        }

        if i == 1 {
            return *points.get(&1).unwrap_or(&0);
        }

        if let Some(&v) = memo.get(&i) {
            return v;
        }

        let gain = points.get(&i).unwrap_or(&0);
        let res = max(gain + go(i - 2, memo, points), go(i - 1, memo, points));
        memo.insert(i, res);
        res
    }

    let max = nums.iter().max().unwrap_or(&0);
    let points = nums.iter().fold(HashMap::new(), |mut acc, &n| {
        *acc.entry(n).or_insert(0) += n;
        acc
    });

    go(*max, &mut HashMap::new(), &points)
}

#[test]
fn example_1() {
    assert_eq!(9, delete_and_earn(vec![2, 2, 3, 3, 3, 4]));
}
