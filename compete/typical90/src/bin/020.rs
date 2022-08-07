use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u32,
        c: u64,
    }
    let yes = a < c.pow(b); // 13.pow(17) < u64::MAX
    println!("{}", if yes { "Yes" } else { "No" });
}
