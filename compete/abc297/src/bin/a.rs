use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        t: [usize; n],
    }
    for x in t.windows(2) {
        if x[1] - x[0] <= d {
            println!("{}", x[1]);
            return;
        }
    }
    println!("{}", -1);
}
