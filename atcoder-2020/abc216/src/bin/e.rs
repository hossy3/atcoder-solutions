use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut result = 0;
    while let Some(x) = a.pop() {
        let m = n - a.len();
        let x0 = a.last().unwrap_or(&0);
        if (x - x0) * m >= k {
            let x0 = x - k / m;
            k -= (x - x0) * m;
            result += m * (x - x0) * (x + x0 + 1) / 2;
            result += x0 * k;
            break;
        }
        k -= (x - x0) * m;
        result += m * (x - x0) * (x + x0 + 1) / 2;
    }
    println!("{}", result);
}
