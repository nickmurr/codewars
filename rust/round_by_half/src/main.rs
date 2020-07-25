fn main() {
    println!("Hello, world!");
}

fn solution(n: f64) -> f64 {
    (n * 2.0).round() / 2.0 as f64
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn sample_tests() {
        assert_eq!(solution(4.2), 4.0);
        assert_eq!(solution(4.4), 4.5);
        assert_eq!(solution(4.6), 4.5);
        assert_eq!(solution(4.8), 5.0);
    }
}
