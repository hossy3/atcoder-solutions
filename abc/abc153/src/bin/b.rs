use proconio::input;

fn main() {
    input! {
        h: usize,
        n: usize,
        a: [usize; n],
    }
    let yes = a.iter().sum::<usize>() >= h;
    println!("{}", if yes { "Yes" } else { "No" });
}
