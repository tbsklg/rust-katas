fn main() {
    println!("{}", likes(&["Peter"]));
}

fn likes(names: &[&str]) -> String {
    match names.len() {
        0 => "no one likes this".to_string(),
        1 => format!("{} likes this", names.first().unwrap()),
        2 => format!(
            "{} and {} like this",
            names.first().unwrap(),
            names.get(1).unwrap()
        ),
        3 => format!(
            "{}, {} and {} like this",
            names.first().unwrap(),
            names.get(1).unwrap(),
            names.get(2).unwrap()
        ),
        _ => format!(
            "{}, {} and {} others like this",
            names.first().unwrap(),
            names.get(1).unwrap(),
            names.len() - 2
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::likes;

    #[test]
    fn example_tests() {
        assert_eq!(likes(&[]), "no one likes this");
        assert_eq!(likes(&["Peter"]), "Peter likes this");
        assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
        assert_eq!(
            likes(&["Max", "John", "Mark"]),
            "Max, John and Mark like this"
        );
        assert_eq!(
            likes(&["Alex", "Jacob", "Mark", "Max"]),
            "Alex, Jacob and 2 others like this"
        );
    }
}
