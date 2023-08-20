use std::collections::HashSet;

use proconio::input;

fn build_primes(n: usize) -> Vec<usize> {
    let mut primes = vec![true; n + 1];
    primes[0] = false;
    primes[1] = false;
    for i in 2..=n {
        if !primes[i] {
            continue;
        }
        let m = n / i;
        for j in 2..=m {
            primes[i * j] = false;
        }
    }

    let m = primes.iter().filter(|&&b| b).count();
    let mut result = Vec::with_capacity(m);
    for (i, &b) in primes.iter().enumerate() {
        if b {
            result.push(i);
        }
    }
    result
}

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let primes = build_primes(200);
    let mut set = HashSet::new();
    for &i in &primes {
        set.insert(i);
    }

    let f = || {
        for i in a..=b {
            let yes = (c..=d).into_iter().all(|j| !set.contains(&(i + j)));
            if yes {
                return true;
            }
        }
        false
    };
    let yes = f();
    println!("{}", if yes { "Takahashi" } else { "Aoki" });
}
