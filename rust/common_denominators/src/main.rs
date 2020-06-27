fn main() {}

fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let res = l.iter().fold(1, |acc, &(num, den)| {
        lowest_common_denominator(acc, den / greatest_common_denominator(num, den))
    });
    l.iter().map(|&(num, den)| (num * res / den, res)).collect()
}

fn greatest_common_denominator(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        greatest_common_denominator(b, a % b)
    }
}

fn lowest_common_denominator(a: i64, b: i64) -> i64 {
    a / greatest_common_denominator(a, b) * b
}

fn testing(l: Vec<(i64, i64)>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(convert_fracts(l), exp)
}

#[test]
fn basics_convert_fracts() {
    testing(
        vec![(69, 130), (87, 1310), (3, 4)],
        vec![(18078, 34060), (2262, 34060), (25545, 34060)],
    );
    testing(
        vec![(690, 1300), (87, 1310), (30, 40)],
        vec![(18078, 34060), (2262, 34060), (25545, 34060)],
    );
    testing(
        vec![(69, 138), (80, 1310), (30, 40)],
        vec![(262, 524), (32, 524), (393, 524)],
    );
}
