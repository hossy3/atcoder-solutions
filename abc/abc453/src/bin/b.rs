use proconio::input;

fn main() {
    input! {
        t: usize,
        x: usize,
        a: [usize; t + 1],
    }
    let mut v = vec![];
    for (i, &a0) in a.iter().enumerate() {
        if i == 0 || a0.abs_diff(a[v[v.len() - 1]]) >= x {
            v.push(i);
        }
    }
    for i in v {
        println!("{} {}", i, a[i]);
    }
}
