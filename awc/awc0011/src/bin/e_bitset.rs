use fixedbitset::FixedBitSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); n],
    }

    let mut candidates = vec![FixedBitSet::with_capacity(n); m + 1];
    let mut max_values = vec![0; m + 1];

    for (i, &(a, b)) in ab.iter().enumerate() {
        for j in (0..=(m - a)).rev() {
            if max_values[j + a] < max_values[j] + b {
                max_values[j + a] = max_values[j] + b;
                candidates[j + a] = candidates[j].clone();
                candidates[j + a].insert(i);
            } else if max_values[j + a] == max_values[j] + b {
                let cand = candidates[j].clone();
                candidates[j + a].union_with(&cand);
                candidates[j + a].insert(i);
            }
        }
    }
    // eprintln!("{:?}", &candidates);

    for i in (0..=m).rev() {
        if candidates[i].len() > 0 {
            for j in 0..n {
                let yes = candidates[i].contains(j);
                println!("{}", if yes { "Yes" } else { "No" });
            }
            return;
        }
    }
}
