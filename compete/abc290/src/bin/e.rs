use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let mut v = vec![0; n];
    for &x in &a {
        v[x] += 1;
    }
    let mut count = 0;
    for w in 1..=(n / 2) {
        let l = a[w - 1];
        let r = a[n - w];
        count += w * (n - 2 * (w - 1) - v[l]);
        count += w * (n - 2 * (w - 1) - v[r]);
        if l != r {
            count -= w;
        }
        v[l] -= 1;
        v[r] -= 1;
    }
    println!("{}", count);

    // // slow implementation:
    // for l in 0..(n - 1) {
    //     for r in (l + 1)..n {
    //         if a[l] != a[r] {
    //             count += (l + 1).min(n - r);
    //         }
    //     }
    // }
}
