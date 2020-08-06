use std::iter::FromIterator;
use std::iter::Iterator;

fn main() {
    descending_order(1286283);
}

fn descending_order(x: u64) -> u64 {
    let my_str: &str = &(format!("{:?}", x))[..];
    let reversed = my_str.chars().rev().collect::<String>();

    let mut chars: Vec<char> = reversed.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    let s: String = chars.into_iter().collect();
    let my_num: u64 = s.parse().unwrap();
    my_num
}

fn descending_order2(x: u64) -> u64 {
    let mut result = x.to_string().chars().collect::<Vec<char>>();
    result.sort_by(|a, b| b.cmp(a));
    String::from_iter(result).parse::<u64>().unwrap()
}

#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}
