use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    let yes = s < t;
    println!("{}", if yes { "Yes" } else { "No" });
}
