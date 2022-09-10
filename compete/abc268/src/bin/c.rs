use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }
    let mut a = vec![0usize; n];
    for (i, &x) in p.iter().enumerate() {
        a[(n + x - i + 1) % n] += 1;
        a[(n + x - i) % n] += 1;
        a[(n + x - i - 1) % n] += 1;
    }
    let score = a.iter().fold(0, |acc, x| acc.max(*x));
    println!("{}", score);
}
