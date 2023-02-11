use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut v = vec![vec![false; n]; m];
    for i in 0..m {
        input! {
            a: [Usize1],
        }
        for j in a {
            v[i][j] = true;
        }
    }

    let mut count = 0;
    for i in 1..(1 << m) {
        let mut a = vec![false; n];
        for j in 0..m {
            if ((1 << j) & i) > 0 {
                for k in 0..n {
                    a[k] = a[k] || v[j][k];
                }
            }
        }
        if a.iter().all(|&x| x) {
            count += 1;
        }
    }
    println!("{}", count);
}
