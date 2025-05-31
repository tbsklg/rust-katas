use std::collections::BTreeSet;

fn main() {
    println!("Hello, world!");
}

fn find_odd(arr: &[i32]) -> i32 {
    arr.iter()
        .fold(BTreeSet::new(), |mut acc, curr| {
            if !acc.insert(*curr) {
                acc.remove(curr);
            }

            acc
        })
        .pop_first()
        .unwrap()
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
