fn main() {
   get_count("abcdefg");
}

fn get_count(input: &str) -> usize {
    input.chars().filter(is_vowel).count()
}

fn is_vowel(input: &char) -> bool {
    "aeiou".contains(*input)
}

#[test]
fn my_tests() {
  assert_eq!(get_count("abracadabra"), 5);
}
