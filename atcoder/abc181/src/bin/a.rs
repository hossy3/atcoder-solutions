use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let yes = n % 2 == 0;
    println!("{}", if yes { "White" } else { "Black" });
}
