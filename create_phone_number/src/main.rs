fn main() {
    println!("Hello, world!");
}

fn create_phone_number(numbers: &[u8]) -> String {
    match numbers {
        [a, b, c, d, e, f, g, h, i, j] => {
            format!("({}{}{}) {}{}{}-{}{}{}{}", a, b, c, d, e, f, g, h, i, j)
        }
        _ => "Invalid input".to_string(),
    }
}

#[test]
fn returns_expected() {
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
        "(123) 456-7890"
    );
    assert_eq!(
        create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
        "(111) 111-1111"
    );
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
        "(123) 456-7899"
    );
}
