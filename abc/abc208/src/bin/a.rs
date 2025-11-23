use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let yes = a <= b && b <= 6 * a;
    println!("{}", if yes { "Yes" } else { "No" });
}
