use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        vp: [(usize, usize); n],
    }
    let mut alcohol = 0;
    for (i, &(v, p)) in vp.iter().enumerate() {
        alcohol += v * p;
        if alcohol > x * 100 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
