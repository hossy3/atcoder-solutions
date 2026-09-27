use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let yes = w * 10000 >= 25 * h * h;
    println!("{}", if yes { "Yes" } else { "No" });
}
