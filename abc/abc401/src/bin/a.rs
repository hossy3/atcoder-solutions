use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let yes = 200 <= n && n <= 299;
    println!("{}", if yes { "Success" } else { "Failure" });
}
