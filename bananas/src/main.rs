#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

fn main() {
    println!("{:?}", bananas("bbananana"));
}

// https://www.codewars.com/kata/5917fbed9f4056205a00001e/train/rust
fn bananas(s: &str) -> Vec<String> {
    mask_chars(s, "banana")
}

fn mask_chars(s: &str, b: &str) -> Vec<String> {
    match (s, b) {
        (xs, "") => vec!["-".repeat(xs.len())],
        ("", &_) => vec![],
        (xs, bs) => {
            let (x, xs) = xs.split_at(1);
            let (y, bs) = bs.split_at(1);

            let mut res = mask_chars(xs, b)
                .iter()
                .map(|w| format!("-{w}"))
                .collect::<Vec<_>>();

            if x == y {
                res.extend(mask_chars(xs, bs).iter().map(|w| format!("{x}{w}")));
            }

            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bananas;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    fn do_test(s: &str, expected: Vec<&str>) {
        let actual: HashSet<String> = HashSet::from_iter(bananas(s));
        assert_eq!(
            actual,
            HashSet::from_iter(expected.iter().map(|x| x.to_string())),
            "s = \"{s}\""
        );
    }

    #[test]
    fn example_0() {
        do_test("banann", Vec::new());
    }
    #[test]
    fn example_1() {
        do_test("banana", vec!["banana"]);
    }
    #[test]
    fn example_2() {
        do_test(
            "bbananana",
            vec![
                "b-an--ana",
                "-banana--",
                "-b--anana",
                "b-a--nana",
                "-banan--a",
                "b-ana--na",
                "b---anana",
                "-bana--na",
                "-ba--nana",
                "b-anan--a",
                "-ban--ana",
                "b-anana--",
            ],
        );
    }
    #[test]
    fn example_3() {
        do_test("bananaaa", vec!["banan-a-", "banana--", "banan--a"]);
    }
    #[test]
    fn example_4() {
        do_test(
            "bananana",
            vec![
                "ban--ana", "ba--nana", "bana--na", "b--anana", "banana--", "banan--a",
            ],
        );
    }
}
