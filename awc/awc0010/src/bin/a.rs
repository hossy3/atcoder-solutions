use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let yes = a.iter().sum::<usize>() >= m;
    println!("{}", if yes { "Yes" } else { "No" });
}
