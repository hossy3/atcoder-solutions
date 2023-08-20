use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let yes = a * 60 + b <= c * 60 + d;
    println!("{}", if yes { "Takahashi" } else { "Aoki" });
}
