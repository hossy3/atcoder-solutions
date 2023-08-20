use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut v = Vec::with_capacity(n);
    for &a in &a {
        let prev = v.last().unwrap_or(&(0, 0));
        let x = (prev.1 + a, prev.0.max(prev.1));
        v.push(x);
    }
    let last = v.last().unwrap_or(&(0, 0));
    let score = last.0.max(last.1);
    println!("{}", score);
}
