use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }
    let yes = x.abs_diff(y) < 3;
    println!("{}", if yes { "Yes" } else { "No" });
}
