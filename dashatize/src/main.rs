fn main() {
    dashatize(274);
}

fn dashatize(n: i64) -> String {
    if n == 0 {
        return "0".to_string();
    }

    digits(n.abs())
        .iter()
        .map(|&x| dash(x))
        .collect::<Vec<String>>()
        .join("")
        .replace("--", "-")
        .trim_matches('-')
        .to_string()
}

fn dash(n: i64) -> String {
    match n % 2 == 0 {
        true => n.to_string(),
        false => format!("-{n}-"),
    }
}

fn digits(n: i64) -> Vec<i64> {
    match n {
        0 => vec![],
        _ => [digits(n / 10), [n % 10].to_vec()].concat(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(dashatize(274), "2-7-4");
        assert_eq!(dashatize(5311), "5-3-1-1");
        assert_eq!(dashatize(86320), "86-3-20");
        assert_eq!(dashatize(974302), "9-7-4-3-02");
    }

    #[test]
    fn weird() {
        assert_eq!(dashatize(0), "0");
        assert_eq!(dashatize(-1), "1");
        assert_eq!(dashatize(-28369), "28-3-6-9");
    }

    #[test]
    fn it_should_calculate_digits() {
        assert_eq!(digits(123), vec![1, 2, 3])
    }
}
