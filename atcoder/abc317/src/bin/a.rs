use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        x: usize,
        p: [usize; n],
    }

    for (i, &p) in p.iter().enumerate() {
        if h + p >= x {
            println!("{}", i + 1);
            return;
        }
    }
}
