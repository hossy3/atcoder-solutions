use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: usize,
    }
    let x = n - t - a;
    let yes = t > a + x || a > t + x;
    println!("{}", if yes { "Yes" } else { "No" });
}
