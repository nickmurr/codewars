// https://www.codewars.com/kata/5552101f47fc5178b1000050/train/rust

fn main() {
    let res = dig_pow_70(46288, 3);
    println!("{}", res)
}

fn dig_pow(n: i64, p: i32) -> i64 {
    let digits: Vec<_> = n
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    let mut sum: i64 = 0;

    for (index, d) in digits.iter().enumerate() {
        sum += i64::pow(*d as i64, (p + index as i32) as u32)
    }
    let k: i64 = sum / n;
    if sum == n * k {
        return k;
    }

    return -1;
}

fn dig_pow_69(n: i64, p: i32) -> i64 {
    let r: i64 = n
        .to_string()
        .chars()
        .map(|c| (c as i64) - 48)
        .enumerate()
        .map(|(i, d)| i64::pow(d, p as u32 + i as u32))
        .sum();

    match r % n {
        0 => r / n,
        _ => -1,
    }
}

fn dig_pow_70(n: i64, p: i32) -> i64 {
    let mut sm = 0;
    let mut m = n;
    let mut pp: u32 = ((n as f64).log10() as u32) + (p as u32);
    while m > 0 {
        sm += (m % 10).pow(pp);
        m = m / 10;
        pp -= 1;
    }
    if sm % n == 0 {
        return sm / n;
    } else {
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i64, p: i32, exp: i64) -> () {
        println!(" n: {:?};", n);
        println!("p: {:?};", p);
        let ans = dig_pow(n, p);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(89, 1, 1);
        dotest(92, 1, -1);
        dotest(46288, 3, 51);
        dotest(114, 3, 9);
        dotest(46288, 5, -1);
        dotest(135, 1, 1);
        dotest(175, 1, 1);
        dotest(518, 1, 1);
        dotest(598, 1, 1);
        dotest(1306, 1, 1);
        dotest(2427, 1, 1);
        dotest(2646798, 1, 1);
        dotest(3456789, 1, -1);
        dotest(3456789, 5, -1);
        dotest(198, 1, 3);
        dotest(249, 1, 3);
        dotest(1377, 1, 2);
        dotest(1676, 1, 1);
        dotest(695, 2, 2);
        dotest(1878, 2, 19);
        dotest(7388, 2, 5);
        dotest(47016, 2, 1);
        dotest(542186, 2, 1);
        dotest(261, 3, 5);
        dotest(1385, 3, 35);
        dotest(2697, 3, 66);
        dotest(6376, 3, 10);
        dotest(6714, 3, 1);
        dotest(63760, 3, 1);
        dotest(63761, 3, 1);
        dotest(132921, 3, 4);
        dotest(10383, 6, 12933);
    }

    extern crate rand;
    use self::rand::{thread_rng, Rng};

    fn dig_pow_70(n: i64, p: i32) -> i64 {
        let mut sm = 0;
        let mut m = n;
        let mut pp: u32 = ((n as f64).log10() as u32) + (p as u32);
        while m > 0 {
            sm += (m % 10).pow(pp);
            m = m / 10;
            pp -= 1;
        }
        if sm % n == 0 {
            return sm / n;
        } else {
            return -1;
        }
    }

    #[test]
    fn random_tests() {
        let p1 = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 89, 135, 175, 198, 249, 518, 598, 1306, 1377, 1676, 2427,
            2646798, 2646799,
        ];
        let p2 = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 43, 48, 63, 695, 1878, 7388, 47016, 542186, 542187,
        ];
        let p3 = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 24, 114, 261, 1385, 2697, 6376, 6714, 46288, 63760, 63761,
            132921, 132922,
        ];
        let p4 = vec![
            9, 48, 257, 376, 448, 871, 1289, 1559, 2570, 15590, 955456, 9001, 9002, 9003, 9004,
        ];
        let p5 = vec![
            24, 41, 63, 604, 699, 978, 1043, 1634, 4424, 6040, 10393, 11646, 14841, 16340, 245439,
            442617,
        ];
        let mut rng = thread_rng();
        for _ in 0..10 {
            let mut i: usize = rng.gen_range(0, p1.len()) as usize;
            let mut n: i64 = p1[i];
            println!(" n1: {:?};", n);
            let mut sol = dig_pow_70(n, 1);
            dotest(n, 1, sol);

            i = rng.gen_range(0, p2.len()) as usize;
            n = p2[i];
            println!(" n2: {:?};", n);
            sol = dig_pow_70(n, 2);
            dotest(n, 2, sol);

            i = rng.gen_range(0, p3.len()) as usize;
            n = p3[i];
            println!(" n3: {:?};", n);
            sol = dig_pow_70(n, 3);
            dotest(n, 3, sol);

            i = rng.gen_range(0, p4.len()) as usize;
            n = p4[i];
            println!(" n4: {:?};", n);
            sol = dig_pow_70(n, 4);
            dotest(n, 4, sol);

            i = rng.gen_range(0, p5.len()) as usize;
            n = p5[i];
            println!(" n5: {:?};", n);
            sol = dig_pow_70(n, 5);
            dotest(n, 5, sol);
        }
    }
}
