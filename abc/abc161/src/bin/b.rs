use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let sum: usize = a.iter().sum();
    let count = a.iter().filter(|&&x| x * 4 * m >= sum).count();
    let yes = count >= m;
    println!("{}", if yes { "Yes" } else { "No" });
}
