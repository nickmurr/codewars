use std::collections::HashMap;

fn main() {
    let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
    let mut c = vec!["A", "B", "C", "D"];
    stock_list(b, c);
}

fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.len() == 0 {
        return "".to_string();
    }

    let mut h: HashMap<&str, i32> = HashMap::new();

    for val in list_art.iter() {
        let r: Vec<&str> = val.split_whitespace().collect();
        let category: &str = &r[0][..1];
        let num: i32 = r[1].to_string().parse().unwrap();

        *h.entry(category).or_insert(0) += num;
    }

    let mut arr: Vec<String> = Vec::new();
    for cat in list_cat.iter() {
        arr.push(format!("({} : {})", cat, h.entry(cat).or_default()))
    }

    return arr.join(" - ");
}

fn stock_list_ba(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.len() == 0 || list_cat.len() == 0 {
        return "".to_string();
    }
    let mut res = "".to_string();
    for cat in list_cat {
        let mut total = 0;
        for i in 0..list_art.len() {
            let book = list_art[i];
            if &book[0..1] == &cat[0..1] {
                let u = book.split(" ").collect::<Vec<&str>>()[1];
                total += u.parse::<i32>().unwrap();
            }
        }
        if res.len() != 0 {
            res += &format!(" {} ", "-");
        }
        res += &format!("({} : {})", cat, total);
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
        println!("list_art: {:?};", list_art);
        println!("list_cat: {:?};", list_cat);
        let ans = stock_list(list_art, list_cat);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let mut c = vec!["A", "B", "C", "D"];
        dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");

        b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        c = vec!["A", "B"];
        dotest(b, c, "(A : 200) - (B : 1140)");
    }
}
