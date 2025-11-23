use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        t: [usize; n],
    }

    let mut cur = 0usize;
    for t in t {
        cur = cur.max(t) + a;
        println!("{cur}");
    }
}
