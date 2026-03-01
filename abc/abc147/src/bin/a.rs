use proconio::input;

fn main() {
    input! {
        a1: usize,
        a2: usize,
        a3: usize,
    }
    let yes = a1 + a2 + a3 >= 22;
    println!("{}", if yes { "bust" } else { "win" });
}
