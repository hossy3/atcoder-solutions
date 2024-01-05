use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i64; n],
    }
    let mut k = 0;
    for i in 0..(n - 1) {
        if p[i] > p[i + 1] {
            k = i;
        }
    }
    let mut prev = p[k];
    for i in (k + 1)..n {
        if prev == p[k] || (prev < p[i] && p[i] < p[k]) {
            prev = p[i];
        }
    }
    let mut v = Vec::new();
    for i in k..n {
        if p[i] != prev {
            v.push(p[i]);
        }
    }
    v.sort_by(|a, b| b.cmp(a));
    for i in 0..k {
        print!("{} ", p[i]);
    }
    print!("{} ", prev);
    for i in 0..(v.len() - 1) {
        print!("{} ", v[i]);
    }
    print!("{}", v.last().unwrap());
}
