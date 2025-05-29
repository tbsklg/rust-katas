fn main() {
    println!("{}", likes(&["Peter"]));
}

fn likes(names: &[&str]) -> String {
    match names {
        [] => "no one likes this".to_string(),
        [x] => format!("{} likes this", x),
        [x, y] => format!("{} and {} like this", x, y),
        [x, y, z] => format!("{}, {} and {} like this", x, y, z),
        [x, y, ys @ ..] => format!("{}, {} and {} others like this", x, y, ys.len()),
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
