fn main() {}

#[allow(dead_code)]
fn comp_ritiek(mut a: Vec<i64>, mut b: Vec<i64>) -> bool {
    a = a.iter().map(|x| x.pow(2)).collect();
    a.sort();
    b.sort();
    a == b
}

#[allow(dead_code)]
fn comp_unnamed(mut a: Vec<i64>, mut b: Vec<i64>) -> bool {
    for x in a.iter_mut() {
        *x *= *x;
    }
    a.sort();
    b.sort();
    a == b
}

#[allow(dead_code)]
fn comp(a: Vec<i64>, mut b: Vec<i64>) -> bool {
    if a.len() == 0 && b.len() == 0 {
        return true;
    }
    if a.len() != b.len() {
        return false;
    }
    for f in a.iter() {
        if !b.contains(&(f * f)) {
            return false;
        }
        let index = b.iter().position(|x| *x == f * f).unwrap();
        b.remove(index);
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) {
        assert_eq!(comp(a, b), exp)
    }

    #[test]
    fn tests_comp() {
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![
            11 * 11,
            121 * 121,
            144 * 144,
            19 * 19,
            161 * 161,
            19 * 19,
            144 * 144,
            19 * 19,
        ];
        testing(a1, a2, true);
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![
            11 * 21,
            121 * 121,
            144 * 144,
            19 * 19,
            161 * 161,
            19 * 19,
            144 * 144,
            19 * 19,
        ];
        testing(a1, a2, false);
    }
}
