fn main() {
    println!("{:?}", ips_between("10.0.0.0", "10.0.0.50"));
}

fn ips_between(start: &str, end: &str) -> u32 {
    as_32_bit(end) - as_32_bit(start)
}

fn as_32_bit(ip: &str) -> u32 {
    ip.split('.')
        .map(|x| x.parse::<u32>().unwrap())
        .fold(0, |acc, x| (acc << 8) + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(ips_between("10.0.0.0", "10.0.0.50"), 50);
        assert_eq!(ips_between("20.0.0.10", "20.0.1.0"), 246);
    }
}
