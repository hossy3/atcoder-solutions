use proconio::{input, marker::Chars};

fn f(mut i: usize, n: usize) -> [usize; 10] {
    let mut a = [0usize; 10];
    let n = 3.min(n);
    for _ in 0..n {
        a[i % 10] += 1;
        i /= 10;
    }
    a
}

fn main() {
    input! {
        s: Chars,
    }
    let mut a = [0usize; 10];
    for &c in &s {
        let i = c as usize - '0' as usize;
        a[i] += 1;
    }

    let yes = (0..125).any(|i| std::iter::zip(a, f(i * 8, s.len())).all(|(x, y)| x >= y));
    println!("{}", if yes { "Yes" } else { "No" });
}
