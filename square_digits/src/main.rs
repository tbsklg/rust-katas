fn main() {
    println!("{:?}", square_digits(9119));
}

fn square_digits(num: u64) -> u64 {
    digits(&num)
        .iter()
        .map(|d| d * d)
        .map(|d| d.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

fn digits(num: &u64) -> Vec<u64> {
    num.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as u64)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::square_digits;

    #[test]
    fn test_square_digits() {
        assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
    }
}
