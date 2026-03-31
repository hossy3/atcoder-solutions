use proconio::input;

fn main() {
    input! {
        mut l: usize,
        mut r: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    for (l0, r0) in lr {
        l = l.max(l0);
        r = r.min(r0);
        println!("{}", (r + 1).saturating_sub(l));
    }
}
