use proconio::input;

fn f(cur: usize, rest: usize, p: usize, q: usize, a: &[usize]) -> usize {
    if rest == 0 {
        a.iter().filter(|&&x| (cur * x) % p == q).count()
    } else {
        let mut count = 0;
        for (i, &x) in a[..(a.len() - rest)].iter().enumerate() {
            count += f((cur * x) % p, rest - 1, p, q, &a[(i + 1)..]);
        }
        count
    }
}

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n]
    }
    let cur = 1;
    let rest = 4;
    let count = f(cur, rest, p, q, &a);
    println!("{count}");
}
