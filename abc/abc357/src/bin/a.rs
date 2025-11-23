use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: usize,
        h: [usize; n],
    }

    for (i, &x) in h.iter().enumerate() {
        if m < x {
            println!("{i}");
            return;
        }
        m -= x;
    }
    println!("{n}");
}
