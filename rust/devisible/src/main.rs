fn main() {
    println!("Hello, world!");
}

fn is_divisible(n: i32, x: i32, y: i32) -> bool {
    n % x == 0 && n % y == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(is_divisible(3, 3, 4), false);
        assert_eq!(is_divisible(12, 3, 4), true);
        assert_eq!(is_divisible(8, 3, 4), false);
        assert_eq!(is_divisible(48, 3, 4), true);
        assert_eq!(is_divisible(100, 5, 10), true);
        assert_eq!(is_divisible(100, 5, 3), false);
        assert_eq!(is_divisible(4, 4, 2), true);
        assert_eq!(is_divisible(5, 2, 3), false);
        assert_eq!(is_divisible(17, 17, 17), true);
        assert_eq!(is_divisible(17, 1, 17), true);
    }

    fn sol(n: i32, x: i32, y: i32) -> bool {
        n % x == 0 && n % y == 0
    }

    #[test]
    fn random_tests() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let n = rng.gen_range(1, 101);
            let x = rng.gen_range(1, 6);
            let y = rng.gen_range(1, 11);
            assert_eq!(is_divisible(n, x, y), sol(n, x, y));
        }
    }
}
