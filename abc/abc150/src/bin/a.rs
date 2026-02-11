use proconio::input;

fn main() {
    input! {
        k: usize,
        x: usize,
    }
    let yes = k * 500 >= x;
    println!("{}", if yes { "Yes" } else { "No" });
}
