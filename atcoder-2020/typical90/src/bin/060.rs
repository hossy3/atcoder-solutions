use proconio::input;
use superslice::Ext;

fn f(a: &[usize]) -> Vec<usize> {
    let mut v = vec![0; a.len()];
    let mut p = Vec::new();

    for (i, &x) in a.iter().enumerate() {
        let j = p.upper_bound(&(x - 1));
        if j == p.len() {
            p.push(x);
        } else if p[j] > x {
            p[j] = x;
        }
        v[i] = j + 1;
    }
    v
}

fn main() {
    input! {
        mut a: [usize],
    }
    let v0 = f(&a);
    a.reverse();
    let v1 = f(&a);
    let count = (0..a.len())
        .map(|i| v0[i] + v1[a.len() - i - 1] - 1)
        .max()
        .unwrap_or(0);
    println!("{}", count);
}
