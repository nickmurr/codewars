fn main() {
    println!("Hello, world!");
}

pub fn remove_char_mine(s: &str) -> String {
    // Your code here!
    let mut x = String::from(s);
    x.remove(0);
    x.remove(x.len() - 1);
    String::from(x)
}

pub fn remove_char(s: &str) -> String {
    s[1..s.len() - 1].to_string()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::remove_char;

    #[test]
    fn sample_cases() {
        assert_eq!(remove_char("eloquent"), "loquen");
        assert_eq!(remove_char("country"), "ountr");
        assert_eq!(remove_char("person"), "erso");
        assert_eq!(remove_char("place"), "lac");
        assert_eq!(remove_char("ok"), "");
        assert_eq!(remove_char("ooopsss"), "oopss");
    }
}
