use proconio::input;

fn main() {
    input! {
        r: usize,
        x: usize,
    }
    let yes = (x == 1 && 1600 <= r && r <= 2999) || (x == 2 && 1200 <= r && r <= 2399);
    println!("{}", if yes { "Yes" } else { "No" });
}
