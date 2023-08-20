use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n],
        b: [i64; m],
    }
    a.sort();
    let mut result = std::i64::MAX;
    for &x in &b {
        match a.binary_search(&x) {
            Ok(_) => {
                result = 0;
                break;
            }
            Err(i) => {
                if i > 0 {
                    result = result.min((a[i - 1] - x).abs());
                }
                if i < a.len() {
                    result = result.min((a[i] - x).abs());
                }
            }
        }
    }
    println!("{}", result);
}
