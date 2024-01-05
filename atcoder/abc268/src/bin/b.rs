use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    let yes = t.find(&s) == Some(0);
    println!("{}", if yes { "Yes" } else { "No" });
}
