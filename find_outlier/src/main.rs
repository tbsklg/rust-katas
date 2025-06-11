fn main() {
    println!("Hello, world!");
}

fn find_outlier(values: &[i32]) -> i32 {
    let evens = values.iter().filter(|x| *x % 2 == 0).collect::<Vec<_>>();
    let odds = values.iter().filter(|x| *x % 2 != 0).collect::<Vec<_>>();

    if evens.len() == 1 {
        return *evens[0];
    }

    *odds[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let t1 = [2,6,8,-10,3];
        let t2 = [206847684,1056521,7,17,1901,21104421,7,1,35521,1,7781];
        let t3 = [std::i32::MAX, 0, 1];
        assert_eq!(3, find_outlier(&t1));
        assert_eq!(206847684, find_outlier(&t2));
        assert_eq!(0, find_outlier(&t3));
    }
}

