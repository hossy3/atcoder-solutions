use proconio::input;
use std::collections::HashMap;

// 0 を除く
fn f0(a: &[usize]) -> usize {
    let Some(&n) = a.iter().max() else { 
        return 0; 
    };
    let mut primes = vec![vec![]; n + 1];
    for i in 2..=n {
        if primes[i].len() > 0 {
            continue;
        }
        let m = n / i;
        for j in 2..=m {
            primes[i * j].push(i);
        }
    }

    let mut map = HashMap::new();
    for &i in a {
        let mut i0 = i;
        for &prime in &primes[i] {
            while i0 % (prime * prime) == 0 {
                i0 /= prime * prime;
            }
        }
        *map.entry(i0).or_insert(0usize) += 1;
    }

    let result: usize = map.iter().map(|(_, x)| x * (x - 1) / 2).sum();
    result
}

// 0 のみ処理
fn f(a: &[usize]) -> usize {
    let num0 = a.iter().filter(|&&x| x == 0).count();
    let a0: Vec<_> = a.iter().filter(|&&x| x != 0).map(|&x| x).collect();
    let base = if num0 == 0 {
        0
    } else if num0 == 1 {
        a0.len()
    } else {
        num0 * (num0 - 1) / 2 + num0 * a0.len()
    };
    base + f0(&a0)
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let result = f(&a);
    println!("{result}");
}
