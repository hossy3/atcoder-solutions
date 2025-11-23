use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    let yes = s == "AtCoder" && t == "Land";
    println!("{}", if yes { "Yes" } else { "No" });
}
