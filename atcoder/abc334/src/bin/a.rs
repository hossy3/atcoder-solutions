use proconio::input;

fn main() {
    input! {
        b: usize,
        g: usize
    }
    let yes = b > g;
    println!("{}", if yes { "Bat" } else { "Glove" });
}
