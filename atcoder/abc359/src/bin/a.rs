use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let result = s.iter().filter(|&s| s == "Takahashi").count();
    println!("{result}");
}
