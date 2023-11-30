use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let yes = if c == 0 { a > b } else { a >= b };
    println!("{}", if yes { "Takahashi" } else { "Aoki" });
}
