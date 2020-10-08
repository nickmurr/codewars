fn main() {
    let a = last_digit("3715290469715693021198967285016729344580685479654510946723", "68819615221552997273737174557165657483427362207517952651");
    let b = last_digit("1606938044258990275541962092341162602522202993782792835301376", "2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376");
    let c = last_digit("4", "1");
    let d = last_digit("4", "2");
    let e = last_digit("9", "7");
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("d = {}", d);
    println!("e = {}", e);
}

fn last_digit(str1: &str, str2: &str) -> i32 {
    if str2 == "0" { return 1; }

    let num = str1.chars().rev().take(1).collect::<String>().parse::<i32>().unwrap();
    let pow = str2.chars().rev().take(2).collect::<String>().chars().rev().collect::<String>().parse::<i32>().unwrap() % 4;

    if num != 0 && num % 2 == 0 && pow == 0 { return 6; }
    if num != 0 && num % 2 != 0 && pow == 0 && num != 5 { return 1; }

    match num {
        1 | 2 | 3 | 4 | 7 | 8 | 9 => num.pow(pow as u32).to_string().chars().rev().take(1).collect::<String>().parse::<i32>().unwrap(),
        5 => 5,
        6 => 6,
        _ => 0
    }
}

fn last_digit2(str1: &str, str2: &str) -> i32 {
    if str2 == "0" { return 1; }
    let x = str1.chars().last().unwrap().to_digit(10).unwrap();
    let m = str2.chars().fold(0, |a, x| (a * 10 + x.to_digit(10).unwrap()) % 4);
    let exp = if m == 0 { 4 } else { m };
    (x.pow(exp) % 10) as i32
}
