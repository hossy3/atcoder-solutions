use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let sum = a.iter().sum::<usize>();
    let yes = sum % n == 0;
    println!("{}", if yes { "Yes" } else { "No" });
}
