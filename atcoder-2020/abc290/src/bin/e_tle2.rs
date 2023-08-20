use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut count = 0;
    for l in 0..(n - 1) {
        for r in (l + 1)..n {
            if a[l] != a[r] {
                count += (l + 1).min(n - r);
            }
        }
    }
    println!("{}", count);
}
