use proconio::input;

fn main() {
    input! {
        s: usize,
        w: usize,
    }
    let yes = s <= w;
    println!("{}", if yes { "unsafe" } else { "safe" });
}
