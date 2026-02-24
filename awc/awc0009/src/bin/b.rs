use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: usize,
        c: usize,
        hp: [(usize, usize); n],
    }

    let mut count = 0usize;
    for (h, p) in hp {
        if s >= h {
            s -= h;
            s += p;
        } else {
            count += 1;
        }
    }
    let result = count * c;
    println!("{result}");
}
