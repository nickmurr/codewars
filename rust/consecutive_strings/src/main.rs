use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    let mut longest = 0;
    let mut res = "".to_string();
    let mut m: HashMap<usize, String> = HashMap::new();

    if strarr.len() == 0 || k > strarr.len() || k == 0 {
        return res;
    }

    for (i, s) in strarr.iter().enumerate() {
        if s.len() > longest {
            longest = s.len();
            res = strarr[i].to_string()
        }
    }

    if k == 1 {
        return res;
    }

    longest = 0;

    for (i, _) in strarr.iter().enumerate() {
        let mut temp_longest: Vec<&str> = vec![];

        for n in i..k + i {
            temp_longest.push(strarr[n]);
        }

        let temp_longest_joined = temp_longest.join("");

        if !m.keys().any(|val| *val == temp_longest_joined.len()) {
            m.insert(temp_longest_joined.len(), temp_longest_joined);
        }

        if i == strarr.len() - k {
            break;
        }
    }

    for i in m {
        if i.0 > longest {
            longest = i.0;
            res = i.1;
        }
    }
    res
}

fn longest_consec_best(strarr: Vec<&str>, k: usize) -> String {
    let mut result = String::new();

    if k > 0 && strarr.len() >= k {
        for index in 0..strarr.len() - k + 1 {
            let s: String = strarr[index..index + k].join("");
            if s.len() > result.len() {
                result = s;
            }
        }
    }

    result
}

fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
    assert_eq!(&longest_consec_best(strarr, k), exp)
}

#[test]
fn basics_longest_consec() {
    testing(
        vec!["zone", "abigail", "theta", "form", "libe", "zas"],
        2,
        "abigailtheta",
    );
    testing(
        vec![
            "ejjjjmmtthh",
            "zxxuueeg",
            "aanlljrrrxx",
            "dqqqaaabbb",
            "oocccffuucccjjjkkkjyyyeehh",
        ],
        1,
        "oocccffuucccjjjkkkjyyyeehh",
    );
    testing(vec![], 3, "");
    testing(
        vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"],
        3,
        "ixoyx3452zzzzzzzzzzzz",
    );
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
    testing(
        vec![
            "bbxxxyyyeesss",
            "qqzzef",
            "rrjdddwww",
            "ffnnnaaanw",
            "bbppykfll",
            "dddlllbblluuuyy",
            "ejjjffoo",
            "uuuebbgdxxx",
            "rrrppppppijvvvgggww",
            "ppbbbkkkaaaxxxfrrhhhttt",
        ],
        3,
        "uuuebbgdxxxrrrppppppijvvvgggwwppbbbkkkaaaxxxfrrhhhttt",
    );
}
