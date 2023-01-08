use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let yes = a.iter().any(|&y| y == x);
    println!("{}", if yes { "Yes" } else { "No" });
}
