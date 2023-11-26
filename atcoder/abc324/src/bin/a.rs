use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let yes = a.windows(2).all(|v| v[0] == v[1]);
    println!("{}", if yes { "Yes" } else { "No" });
}
