use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    if n * n < m {
        println!("{}", -1);
        return;
    }
    let a_max = (m as f64).sqrt().ceil() as usize;
    let mut result = a_max * a_max;
    for a in (1..=a_max).rev() {
        let b = ((m as f64) / (a as f64)).ceil() as usize;
        if b > n {
            break;
        }
        result = result.min(a * b);
    }
    println!("{}", result);
}
