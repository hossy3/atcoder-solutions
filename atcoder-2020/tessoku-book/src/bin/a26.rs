use proconio::input;

fn build_primes(n: usize) -> Vec<bool> {
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
    primes
}

fn main() {
    input! {
        x: [usize],
    }
    
    let primes = build_primes(*x.iter().max().unwrap());
    for x in &x {
        let yes = primes[*x];
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
