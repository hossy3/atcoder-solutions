use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    let yes = (0..n).all(|i| a[i] == i + 1);
    println!("{}", if yes { "Yes" } else { "No" });
}
