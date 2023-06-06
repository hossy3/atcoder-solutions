use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n],
    }
    let mut l = (0, ab[0].0); // index, rest
    let mut r = (n - 1, ab[n - 1].0); // index, rest
    let mut result = 0.0; // pos
    while l.0 != r.0 {
        let lt = l.1 / ab[l.0].1;
        let rt = r.1 / ab[r.0].1;
        result += ab[l.0].1 * lt.min(rt);
        if lt < rt {
            l = (l.0 + 1, ab[l.0 + 1].0);
            r.1 -= ab[r.0].1 * lt;
        } else {
            l.1 -= ab[l.0].1 * rt;
            r = (r.0 - 1, ab[r.0 - 1].0);
        }
    }
    result += (l.1 + r.1 - ab[l.0].0) * 0.5;
    println!("{}", result);
}
