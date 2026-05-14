use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    }
    let count = a.iter().map(|&a| a / k).sum::<usize>();
    let yes = count >= m;
    println!("{}", if yes { "Yes" } else { "No" });
}
