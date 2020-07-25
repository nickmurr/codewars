use std::collections::HashMap;

macro_rules! hash_map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

fn main() {
    let friends = ["A1", "A2", "A3", "A4", "A5"];
    let fr_towns = hash_map! { "A1" => "X1", "A2"=> "X2", "A3" => "X3", "A4" => "X4" };
    let dst = hash_map! { "X1" => 100.0, "X2" => 200.0, "X3" => 250.0, "X4" => 300.0 };
    println!("{}", tour(&friends, fr_towns, dst));
}

fn tour(frnds: &[&str], fr_twns: HashMap<&str, &str>, dist: HashMap<&str, f64>) -> i32 {
    println!("{:?}", fr_twns);
    println!("{:?}", dist);
    let friends_to_visit: Vec<f64> = frnds
        .iter()
        .rev()
        .filter(|v| fr_twns.contains_key(*v))
        .map(|x| dist[fr_twns[*x]])
        .collect();

    let mut res: f64 = friends_to_visit[0] + friends_to_visit[friends_to_visit.len() - 1];

    for (i, _) in friends_to_visit.iter().enumerate() {
        if i != friends_to_visit.len() - 1 {
            res += sqr(&friends_to_visit[i..i + 2])
        }
    }

    res as i32
}

fn sqr(v: &[f64]) -> f64 {
    (v[0].powf(2.0) - v[1].powf(2.0)).sqrt()
}

// fn tour(frnds: &[&str], fr_twns: HashMap<&str, &str>, dist: HashMap<&str, f64>) -> i32 {
//     let mut ds: f64 = 0.;
//     (frnds.iter().fold(0., |mut sum, fr| {
//         if let Some(twn) = fr_twns.get(fr) {
//             let n_ds = *dist.get(twn).unwrap();
//             if ds > n_ds {
//                 sum += (ds * ds - n_ds * n_ds).sqrt();
//                 ds = n_ds;
//             } else {
//                 sum += (n_ds * n_ds - ds * ds).sqrt();
//                 ds = n_ds;
//             }
//         }
//         sum
//     }) + ds) as i32
// }
