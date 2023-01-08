use proconio::input;

// two-pointer technique (尺取り法)

fn main() {
    input! {
        n: usize,
        k: i64,
        mut a: [i64; n],
    }

    a.sort();
    let mut count = 0;
    let mut r = 1;
    for l in 0..(n - 1) {
        r = r.max(l + 1);
        while r < a.len() && a[r] - a[l] <= k {
            r += 1;
        }
        count += r - l - 1;
    }
    println!("{}", count);
}
