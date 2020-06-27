fn main() {
    // [1970    7880   1000 1.5]
    // [1930.6  7722.4 2000 2.0]

    nb_months(2000, 8000, 1000, 1.5);
}

fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    if old == new {
        return (0, 0);
    }

    if old > new {
        return (0, old - new);
    }

    let mut old_price = old as f64;
    let mut new_price = new as f64;
    let mut percantage = perc;
    let mut tips = 0.0;
    let mut idx = 1;
    let mut savings = 0.0;

    while tips <= 0.0 {
        if idx % 2 == 0 {
            percantage += 0.5;
        }

        let p = (100.0 - percantage) / 100.0;

        old_price *= p;
        new_price *= p;
        savings += saving as f64;
        tips = old_price - new_price + savings;
        idx += 1;
    }

    (idx - 1, tips.round() as i32)
}

fn testing(old: i32, new: i32, saving: i32, perc: f64, exp: (i32, i32)) -> () {
    assert_eq!(nb_months(old, new, saving, perc), exp)
}

#[test]
fn basics_nb_months() {
    testing(2000, 8000, 1000, 1.5, (6, 766));
    testing(12000, 8000, 1000, 1.5, (0, 4000));
    testing(8000, 12000, 500, 1.0, (8, 597));
    testing(18000, 32000, 1500, 1.25, (8, 332));
    testing(7500, 32000, 300, 1.55, (25, 122));
}
