use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [isize; n],
    }
    let yes = x.iter().all(|&x| x < 0);
    println!("{}", if yes { "Yes" } else { "No" });
}
