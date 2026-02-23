use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        t: usize,
        a: [usize; n],
    }
    let yes = a.iter().sum::<usize>() <= (t - s) * 60;
    println!("{}", if yes { "Yes" } else { "No" });
}
