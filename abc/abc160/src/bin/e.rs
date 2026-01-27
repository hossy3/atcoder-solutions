use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
        p: [usize; a],
        q: [usize; b],
        r: [usize; c],
    }

    // p と r だけを使って X..=(X+Y) 個のおいしさ総和の最大を得る。ただし p は X 個以上使えない
    let p: Vec<_> = p.iter().cloned().sorted().rev().take(x).collect();
    let pr: Vec<_> = p.iter().chain(r.iter()).cloned().sorted().rev().collect();
    let mut v0 = vec![0; x + y + 1];
    for (i, &x) in pr[..((x + y).min(pr.len()))].iter().enumerate() {
        v0[i + 1] = v0[i] + x;
    }

    // q だけを使って 0..=Y 個のおいしさ総和の最大を得る
    let q: Vec<_> = q.iter().cloned().sorted().rev().take(y).collect();
    let mut v1 = vec![0; y + 1];
    for (i, &x) in q.iter().enumerate() {
        v1[i + 1] = v1[i] + x;
    }

    let result = (0..=y).map(|i| v0[x + i] + v1[y - i]).max().unwrap();
    println!("{result}");
}
