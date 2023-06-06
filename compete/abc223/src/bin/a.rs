use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    let yes = (x > 0) && (x % 100 == 0);
    println!("{}", if yes { "Yes" } else { "No" });
}
