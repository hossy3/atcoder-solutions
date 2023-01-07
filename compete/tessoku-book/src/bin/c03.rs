use proconio::input;

fn main() {
    input! {
        d: usize,
        x: i64,
        a: [i64; d - 1],
        q: usize,
        st: [(usize, usize); q],
    }
    let mut v = Vec::with_capacity(d);
    v.push(x);
    for &diff in &a {
        v.push(v.last().unwrap() + diff);
    }
    for &(s, t) in &st {
        let xs = v[s - 1];
        let xt = v[t - 1];
        if xs > xt {
            println!("{}", s);
        } else if xs < xt {
            println!("{}", t);
        } else {
            println!("{}", "Same");
        }
    }
}
