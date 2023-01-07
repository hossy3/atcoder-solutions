use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    for s in s.iter().rev() {
        println!("{}", s);
    }
}
