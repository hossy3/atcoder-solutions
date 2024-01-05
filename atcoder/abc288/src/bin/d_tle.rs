use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
        q: usize,
        lr: [(Usize1, Usize1); q],
    }
    
    for (l, r) in lr {
        let mut a = Vec::from(&a[l..=r]);
        for i in 0..=(a.len() - k) {
            for j in (0..k).rev() {
                a[i + j] -= a[i];
            }
        }
        let yes = a[(a.len() - k)..].iter().all(|&x| x == 0);
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
