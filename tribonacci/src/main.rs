use std::collections::HashMap;

fn main() {
    tribonacci(345);
}

fn tribonacci(n: i32) -> i32 {
    fn go(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if n == 0 {
            return 0;
        }

        if n < 3 {
            return 1;
        }

        if let Some(&v) = memo.get(&n) {
            return v;
        }
        
        let res = 
            go(n - 1,memo) + go (n - 2,memo) + go (n - 3, memo);
        
        memo.insert(n, res);
        res
    }

    let mut memo = HashMap::new();
    go(n, &mut memo)
}

#[test]
fn example_1() {
    assert_eq!(1389537, tribonacci(25))
}
