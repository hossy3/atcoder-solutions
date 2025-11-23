use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut dp = (1usize, 1usize); // y = false, y = true
    for s in s {
        if s == "AND" {
            dp = (dp.0 * 2 + dp.1, dp.1);
        } else {
            dp = (dp.0, dp.0 + dp.1 * 2);
        }
    }
    println!("{}", dp.1);
}
