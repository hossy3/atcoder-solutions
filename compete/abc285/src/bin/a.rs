use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let yes = b / 2 == a;
    println!("{}", if yes { "Yes" } else { "No" });
}
