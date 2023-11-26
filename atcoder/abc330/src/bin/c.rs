use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        d: i64,
    }

    let mut result = i64::MAX;
    let x0 = d.sqrt() as i64;
    for x in 0..=x0 {
        let y = ((d - x * x) as f64).sqrt().floor() as i64;
        result = result.min((x * x + y * y - d).abs());
        let y = y + 1;
        result = result.min((x * x + y * y - d).abs());
    }
    println!("{result}");
}
