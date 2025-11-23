use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }
    let yes = (y % 2 == 0) && (x * 2 <= y) && (y <= x * 4);
    println!("{}", if yes { "Yes" } else { "No" });
}
