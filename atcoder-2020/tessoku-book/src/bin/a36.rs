use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let yes = (k >= (n - 1) * 2) && (k % 2 == 0);
    println!("{}", if yes { "Yes" } else { "No" });
}
