use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut result = 0usize;
    let mut i = 1;
    while i <= n {
        let m = n / i;
        if m != n / (i + 1) {
            result += m;
            i += 1;
        } else {
            let i0 = n / m + 1;
            result += (i0 - i) * m;
            i = i0;
        }
    }
    println!("{}", result);
}
