fn main() {
    println!("Hello, world!");
}

// fn rgb(r: i32, g: i32, b: i32) -> String {
//     let mut res: String = String::from("");
//     for &v in [r, g, b].iter() {
//         let mut x = v;
//         if x > 255 {
//             x = 255
//         };
//         if x < 0 {
//             x = 0
//         };
//         res = format!("{}{:02X}", res, x);
//     }
//     res
// }

fn rgb(r: i32, g: i32, b: i32) -> String {
    format!(
        "{0:02X}{1:02X}{2:02X}",
        r.min(255).max(0),
        g.min(255).max(0),
        b.min(255).max(0)
    )
}

// fn rgb(r: i32, g: i32, b: i32) -> String {
//       let clamp = |x: i32| -> i32 { if x < 0 { 0 } else { if x > 255 { 255 } else { x } } };
//     format!("{:02X}{:02X}{:02X}", clamp(r), clamp(g), clamp(b))
// }

macro_rules! compare {
    ( $got : expr, $expected : expr ) => {
        if $got != $expected {
            panic!("Got: {}\nExpected: {}\n", $got, $expected);
        }
    };
}

#[cfg(test)]
mod sample_tests {
    use self::super::*;

    #[test]
    fn tests() {
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(255, 255, 255), "FFFFFF");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(-20, 275, 125), "00FF7D");
    }
}
