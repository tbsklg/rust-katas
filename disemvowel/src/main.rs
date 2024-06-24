fn main() {
    println!("{}",disemvowel("This website is for losers LOL!"));
}

fn disemvowel(s: &str) -> String {
    s.chars().filter(|x| !is_vowel(x)).collect()
}

fn is_vowel(s: &char) -> bool {
    "aeiouAEIOU".contains(*s)
}

#[cfg(test)]
mod tests {
    use super::disemvowel;
    
    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
}
