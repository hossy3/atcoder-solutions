use proconio::input;

fn main() {
    input! {
        a: usize,
        b: u32,
        c: usize,
    }
    let yes = a < c.pow(b); // 13.pow(17) < usize::MAX
    println!("{}", if yes { "Yes" } else { "No" });
}
