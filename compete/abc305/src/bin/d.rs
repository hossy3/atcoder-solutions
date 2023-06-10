use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut v = vec![0; n];
    for i in 0..(n - 1) {
        if i % 2 == 0 {
            v[i + 1] = v[i];
        } else {
            v[i + 1] = v[i] + a[i + 1] - a[i];
        }
    }

    for &(l, r) in &lr {
        let l0 = match a.binary_search(&l) {
            Ok(n) => v[n],
            Err(n) => (v[n - 1] * (a[n] - l) + (v[n] * (l - a[n - 1]))) / (a[n] - a[n - 1]),
        };
        let r0 = match a.binary_search(&r) {
            Ok(n) => v[n],
            Err(n) => (v[n - 1] * (a[n] - r) + (v[n] * (r - a[n - 1]))) / (a[n] - a[n - 1]),
        };
        let result = r0 - l0;
        println!("{}", result);
    }
}
