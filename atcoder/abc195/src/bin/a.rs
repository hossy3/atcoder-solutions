use proconio::input;

fn main() {
    input! {
        m: usize,
        h: usize,
    }
    let yes = h % m == 0;
    println!("{}", if yes { "Yes" } else { "No" });
}
