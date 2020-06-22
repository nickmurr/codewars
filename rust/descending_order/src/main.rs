fn main() {
    descending_order(1286283);
}

fn descending_order(x: u64) -> u64 {
    if x < 10 {
        return x;
    }
    let mut digits: Vec<_> = x
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    digits.sort();
    digits.reverse();
    digits.join(", ");

    return x;

    // let my_str: String = format!("{:?}", x);
    // let reversed = my_str.chars().rev().collect::<String>();
    // return reversed.parse().unwrap();
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
