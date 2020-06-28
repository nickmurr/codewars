fn main() {
    println!("Hello, world!");
}

fn longest_mine(a1: &str, a2: &str) -> String {
    let mut chars: Vec<char> = format!("{}{}", a1, a2).chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    chars.dedup();
    let mut res = "".to_string();
    for r in chars.iter() {
        res = format!("{}{}", res, r);
    }

    return res;
}

fn longest_like(a1: &str, a2: &str) -> String {
    let mut v = (a1.to_owned() + a2).chars().collect::<Vec<_>>();
    v.sort();
    v.dedup();
    v.iter().collect()
}

use std::collections::BTreeSet;
// _best
fn longest(a1: &str, a2: &str) -> String {
    a1.chars()
        .chain(a2.chars())
        .collect::<BTreeSet<char>>()
        .iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing(
            "loopingisfunbutdangerous",
            "lessdangerousthancoding",
            "abcdefghilnoprstu",
        );
    }
}
