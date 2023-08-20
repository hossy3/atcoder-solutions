use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    let sum = a.iter().sum::<usize>() - n / 2;
    let yes = x >= sum;
    println!("{}", if yes { "Yes" } else { "No" });
}
