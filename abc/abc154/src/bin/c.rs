use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort_unstable();
    let yes = a.windows(2).all(|v| v[0] < v[1]);
    println!("{}", if yes { "YES" } else { "NO" });
}
