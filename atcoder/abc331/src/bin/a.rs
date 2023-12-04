use proconio::input;

fn main() {
    input! {
        m0: usize,
        d0: usize,
        mut y: usize,
        mut m: usize,
        mut d: usize,
    }
    d += 1;
    if d > d0 {
        d = 1;
        m += 1;
    }
    if m > m0 {
        m = 1;
        y += 1;
    }
    println!("{y} {m} {d}");
}
