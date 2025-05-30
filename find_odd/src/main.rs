use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn find_odd(arr: &[i32]) -> i32 {
    arr.iter()
        .fold(HashMap::new(), |mut acc, curr| {
            acc.entry(curr).and_modify(|cnt| *cnt += 1).or_insert(1);
            acc
        })
        .iter()
        .find(|(_, v)| *v % 2 != 0)
        .map_or(-1, |v| **v.0)
}

#[cfg(test)]
mod tests {
    use super::find_odd;

    #[test]
    fn basic_tests() {
        assert_eq!(
            find_odd(&[20, 1, -1, 2, -2, 3, 3, 5, 5, 1, 2, 4, 20, 4, -1, -2, 5]),
            5
        );
        assert_eq!(find_odd(&[1, 1, 2, -2, 5, 2, 4, 4, -1, -2, 5]), -1);
        assert_eq!(find_odd(&[20, 1, 1, 2, 2, 3, 3, 5, 5, 4, 20, 4, 5]), 5);
        assert_eq!(find_odd(&[10]), 10);
        assert_eq!(find_odd(&[1, 1, 1, 1, 1, 1, 10, 1, 1, 1, 1]), 10);
        assert_eq!(find_odd(&[5, 4, 3, 2, 1, 5, 4, 3, 2, 10, 10]), 1);
    }
}
