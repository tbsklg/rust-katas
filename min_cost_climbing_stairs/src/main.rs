use std::{cmp::min, collections::HashMap};

fn main() {
    min_cost_climbing_stairs(vec![1, 2, 3, 4]);
}

fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    fn go(i: i32, cost: &Vec<i32>, memo: &mut HashMap<i32, i32>) -> i32 {
        if i < 2 {
            return cost[i as usize];
        }

        if let Some(&v) = memo.get(&i) {
            return v;
        }

        let res = cost[i as usize] + min(go(i - 1, cost, memo), go(i - 2, cost, memo));

        memo.insert(i, res);
        res
    }

    let n = cost.len() as i32 - 1;
    let mut m: HashMap<i32, i32> = HashMap::new();

    min(go(n, &cost, &mut m), go(n - 1, &cost, &mut m))
}

#[test]
fn example_1() {
    assert_eq!(15, min_cost_climbing_stairs(vec![10, 15, 20]))
}

#[test]
fn example_2() {
    assert_eq!(
        6,
        min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
    )
}
