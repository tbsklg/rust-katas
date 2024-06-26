fn main() {
    println!("{}", spin_words("Hey fellow warriors"));
}

fn spin_words(words: &str) -> String {
    words
        .split_whitespace()
        .map(|word| {
            match word.len() >= 5 {
                true => word.chars().rev().collect(),
                false => word.to_string(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
