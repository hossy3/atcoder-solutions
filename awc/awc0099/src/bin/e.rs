use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut results = vec![false; n];
    for _ in 0..m {
        input! {
            k: usize,
            s: [Usize1; k],
        }
        if k == 1 {
            results[s[0]] = true;
        }
    }

    let result = results.iter().filter(|&&b| b).count();
    println!("{result}");
}
