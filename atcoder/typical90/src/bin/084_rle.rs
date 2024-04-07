use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut rle = vec![1];
    for v in s.windows(2) {
        if v[0] == v[1] {
            *rle.last_mut().unwrap() += 1;
        } else {
            rle.push(1);
        }
    }

    let f = |n: usize| n * (n + 1) / 2;
    let count = rle.iter().fold(f(n), |acc, &x| acc - f(x));
    println!("{count}");
}
