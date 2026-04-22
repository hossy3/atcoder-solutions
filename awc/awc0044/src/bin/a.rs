use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: usize,
        mut cs: [(usize, usize); n],
    }

    for (c, s) in cs.iter_mut() {
        let x = m.min(*c - *s);
        *s += x;
        m -= x;
        println!("{s}");
    }
}
