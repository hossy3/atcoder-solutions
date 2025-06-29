use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        mut t: [usize; n],
    }
    t.insert(0, 0);
    let yes = t.windows(2).all(|w| w[1] - w[0] <= s);
    println!("{}", if yes { "Yes" } else { "No" });
}
