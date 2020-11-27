use std::str::FromStr;

fn main() {
    println!("{}", stati("01|15|59, 1|47|16, 01|17|20, 1|32|34, 2|17|17"));
    println!("{}", stati("02|15|59, 2|47|16, 02|17|20, 2|32|34, 2|17|17, 2|22|00, 2|31|41"));
    println!("{}", stati("02|15|59, 2|47|16, 02|17|20, 2|32|34, 2|32|34, 2|17|17"));
}

struct Stat {
    map: Vec<i32>,
}

impl Stat {
    fn new(s: &str) -> Stat {
        let mut map = s
            .split(", ")
            .into_iter()
            .map(|x| Stat::to_sec(&x.split("|").map(|x| i32::from_str(x).unwrap_or(0)).collect()))
            .collect::<Vec<i32>>();

        map.sort();
        Stat { map }
    }

    fn to_sec(x: &Vec<i32>) -> i32 {
        x[2] + (x[1] * 60) + (x[0] * 60 * 60)
    }

    fn range(&self) -> i32 {
        let last = &self.map[self.map.len() - 1];
        let first = &self.map[0];
        last - first
    }

    fn average(&self) -> i32 {
        (self.map.iter().fold(0, |acc, curr| acc + curr) as usize / self.map.len()) as i32
    }

    fn median(&self) -> i32 {
        let half_idx = (&self.map.len() - 1) / 2;
        return match &self.map.len() % 2 {
            0 => (&self.map[half_idx + 1] + &self.map[half_idx]) / 2,
            _ => self.map[half_idx]
        };
    }

    fn string(seconds: i32) -> String {
        let s: i32 = seconds % 60;
        let m: i32 = (seconds - s) / 60 % 60;
        let h: i32 = (seconds - s - (m * 60)) / 3600;
        format!("{:0>#2}|{:0>#2}|{:0>#2}", h, m, s)
    }
}

fn stati(strg: &str) -> String {
    if strg.len() == 0 {
        return String::from("");
    }

    let stat = Stat::new(strg);

    format!("Range: {} Average: {} Median: {}", Stat::string(stat.range()), Stat::string(stat.average()), Stat::string(stat.median()), )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(strg: &str, exp: &str) -> () {
        println!(" str: {:?};", strg);
        let ans = stati(strg);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest("01|15|59, 1|47|16, 01|17|20, 1|32|34, 2|17|17",
               "Range: 01|01|18 Average: 01|38|05 Median: 01|32|34");
        dotest("02|15|59, 2|47|16, 02|17|20, 2|32|34, 2|17|17, 2|22|00, 2|31|41",
               "Range: 00|31|17 Average: 02|26|18 Median: 02|22|00");
    }
}
