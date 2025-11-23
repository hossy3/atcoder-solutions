use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize,
    }
    let yes = a.iter().any(|&x0| x0 == x);
    println!("{}", if yes { "Yes" } else { "No" });
}
