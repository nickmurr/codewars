fn main() {
    println!("{:?}", iter_pi(0.1));
}

fn rnd10(f: f64) -> f64 {
    (f * 1e10).round() / 1e10
}
// public: static string iterPi(double epsilon){
//   if(epsilon == 0.0000001)
//     return "[10000001, 3.1415927536]";  // fix for weird test

//   unsigned int den = 3;
//   long cnt = 1;
//   int prefix = 1;
//   long double pi = 1.0;
//   while(PiApprox::abs(4*pi - M_PI) >= epsilon){
//     prefix = cnt%2 == 0 ? 1 : -1;
//     pi += prefix * (1.0/den);
//     cnt++;
//     den += 2;
//   }

//   string iter = to_string(cnt);
//   stringstream ss;
//   string spi;
//   ss << setprecision(11) << 4*pi;
//   ss >> spi;
//   string s = "["+iter+", "+spi+"]";
//   return s;
// }

fn iter_pi(epsilon: f64) -> (i32, f64) {
    let mut den = 3f64;
    let mut cnt = 1;
    let mut prefix = 1f64;
    let mut pi = 1.0;
    while (4.0 * pi - std::f64::consts::PI).abs() >= epsilon {
        // prefix = cnt%2 == 0 ? 1 : -1;
        prefix = if cnt % 2 == 0 { 1f64 } else { -1f64 };
        pi += prefix * (1.0 / den);
        cnt += 1;
        den += 2.0;
    }

    return (cnt, rnd10(pi * 4.0));
}

fn testing(epsilon: f64, exp: (i32, f64)) -> () {
    assert_eq!(iter_pi(epsilon), exp)
}

#[test]
fn tests_iter_pi() {
    testing(0.1, (10, 3.0418396189));
    testing(0.01, (100, 3.1315929036));
    testing(0.001, (1000, 3.1405926538));
    testing(0.0001, (10000, 3.1414926536));
    testing(0.00001, (100001, 3.1416026535));
    testing(0.000001, (1000001, 3.1415936536));
    testing(0.05, (20, 3.0916238067));
}
