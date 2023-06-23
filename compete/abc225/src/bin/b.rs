use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut v = vec![0; n];
    for &(a, b) in &ab {
        if a != b {
            v[a] += 1;
            v[b] += 1;
        }
    }
    let yes = v.iter().filter(|&&x| x == n - 1).count() == 1
        && v.iter().filter(|&&x| x == 1).count() == n - 1;
    println!("{}", if yes { "Yes" } else { "No" });
}
