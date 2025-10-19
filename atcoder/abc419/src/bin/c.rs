use proconio::input;

fn main() {
    input! {
        n: usize,
        rc: [(i64, i64); n],
    }

    let mut r_min = i64::MAX;
    let mut r_max = i64::MIN;
    let mut c_min = i64::MAX;
    let mut c_max = i64::MIN;
    for &(r, c) in &rc {
        r_min = r_min.min(r);
        r_max = r_max.max(r);
        c_min = c_min.min(c);
        c_max = c_max.max(c);
    }

    let result = ((r_max - r_min + 1) / 2).max((c_max - c_min + 1) / 2);
    println!("{result}");
}
