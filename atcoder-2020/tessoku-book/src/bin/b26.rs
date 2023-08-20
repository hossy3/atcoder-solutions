use proconio::input;

fn build_primes(n: usize) -> Vec<usize> {
    let mut primes = vec![2usize];
    'outer: for i in 3..=n {
        for prime in &primes {
            if i % prime == 0 {
                continue 'outer;
            }
            if prime * prime > i {
                primes.push(i);
                continue 'outer;
            }
        }
        panic!("unreachable");
    }
    primes
}

fn main() {
    input! {
        n: usize,
    }
    let v = build_primes(n);
    for &x in &v {
        println!("{}", x);
    }
}
