use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut count = 0;
    let mut sum = 0;
    let mut r = 0;
    for l in 0..n {
        while r < a.len() && sum + a[r] <= k {
            sum += a[r];
            r += 1;
        }
        sum -= a[l];
        count += r - l;
    }
    println!("{}", count);
}
