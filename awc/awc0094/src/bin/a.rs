use proconio::input;

fn main() {
    input! {
        n: usize,
        mut w: usize,
v: [usize; n],
    }

    for v in v {
        if w >= v {
            w += v;
        }
    }
    println!("{w}");
}
