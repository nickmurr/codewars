fn main() {
    decompose(21, 23);
}

fn decompose(num: u32, denom: u32) -> String {
    if num == 0 {
        return String::from("");
    }

    if num == denom {
        return String::from("1");
    }

    let mut n = num as f64;
    let mut d = denom as f64;

    let mut res = String::new();

    if num > denom {
        res = format!("{}", num / denom);
        n = n % d;
    }

    let mut ef: Vec<f64> = vec![];

    while n != 0.0 {
        let x = (d / n).ceil();
        ef.push(x);
        n = x * n - d;
        d = d * x;
    }

    for (i, v) in ef.iter().enumerate() {
        if i == 0 && res.len() == 0 {
            res = format!("1/{}", *v as i32);
            continue;
        }
        res = format!("{}, 1/{}", res, *v as i32);
    }

    res
}

fn decompose_best(num: u32, denom: u32) -> String {
    let mut ef = Vec::default();
    let (mut dr, mut nr) = (denom as f64, num as f64);

    if num >= denom {
        ef.push(format!("{}", num / denom));
        nr %= dr;
    }
    while nr != 0.0 {
        let x = (dr / nr).ceil() as f64;
        ef.push(format!("1/{:0}", x));
        nr = x * nr - dr;
        dr = dr * x;
    }
    ef.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(num: u32, denom: u32, exp: String) -> () {
        let ans = decompose(num, denom);
        assert_eq!(ans, exp, "Testing: {} {}", num, denom);
    }

    #[test]
    fn basic_tests() {
        testing(3, 4, "1/2, 1/4".to_string());
        testing(12, 4, "3".to_string());
        testing(0, 2, "".to_string());
    }
}
