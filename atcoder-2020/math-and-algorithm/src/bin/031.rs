use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
    }
    let mut v = vec![0usize; n + 3];
    for (i, &x) in s.iter().enumerate() {
        v[i] += x;
        v[i + 2] = v[i + 2].max(v[i]);
        v[i + 3] = v[i];
    }
    let result = v[n];
    println!("{}", result);
}
