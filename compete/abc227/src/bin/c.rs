use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut result = 0usize;
    let i_max = (n as f64).powf(1.0 / 3.0).ceil() as usize;
    for i in 1..=i_max {
        let n = n / i;
        let j_max = n.sqrt();
        for j in i..=j_max {
            let n = n / j;
            if n >= j {
                result += n - j + 1;
            }
        }
    }
    println!("{}", result);
}
