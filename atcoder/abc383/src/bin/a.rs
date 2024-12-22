use proconio::input;

fn main() {
    input! {
        n: usize,
        tv: [(i64, i64); n],
    }
    let mut x = tv[0].1;
    for tv in tv.windows(2) {
        let &[(t0, _), (t1, v1)] = tv else { unreachable!() };
        x = (x - (t1 - t0)).max(0) + v1;
    }
    println!("{x}");
}
