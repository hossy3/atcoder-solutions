use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    const MAX: usize = 100000;
    let mut divisors = vec![vec![]; MAX + 1];
    for i in 2..=MAX {
        if divisors[i].len() == 0 {
            for k in 1..=(MAX / i) {
                divisors[i * k].push(i);
            }
        }
    }

    let mut v = vec![false; MAX + 1];
    for &i in &a {
        for &j in &divisors[i] {
            if !v[j] {
                for k in 1..=(MAX / j) {
                    v[j * k] = true;
                }
            }
        }
    }

    let count = v[1..=m].iter().filter(|&&x| !x).count();
    println!("{}", count);
    for i in 1..=m {
        if !v[i] {
            println!("{}", i);
        }
    }
}
