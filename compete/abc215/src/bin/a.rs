use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let yes = s == "Hello,World!";
    println!("{}", if yes { "AC" } else { "WA" });
}
