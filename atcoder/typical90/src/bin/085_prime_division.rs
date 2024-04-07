use std::collections::HashSet;

use proconio::input;

fn prime_division(mut n: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let mut k = 2;
    while k * k <= n {
        let mut count = 0;
        while n % k == 0 {
            count += 1;
            n /= k;
        }
        if count > 0 {
            result.push((k, count));
        }
        k += if k == 2 { 1 } else { 2 };
    }
    if n > 1 {
        result.push((n, 1));
    }
    result
}

fn f(base: [usize; 3], v: &[(usize, usize)], s: &mut HashSet<[usize; 3]>) {
    if v.len() == 0 {
        let mut base = base;
        base.sort();
        s.insert(base);
    } else {
        let (x, m) = v[0];
        for i in 0..=m {
            for j in 0..=(m - i) {
                let k = m - i - j;
                let b = [
                    base[0] * x.pow(i as u32),
                    base[1] * x.pow(j as u32),
                    base[2] * x.pow(k as u32),
                ];
                f(b, &v[1..], s);
            }
        }
    }
}

fn main() {
    input! {
        k: usize,
    }
    let v = prime_division(k);
    let mut s = HashSet::new();
    f([1, 1, 1], &v, &mut s);
    let result = s.len();
    println!("{result}");
}
