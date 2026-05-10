use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let yes = a.iter().sum::<usize>() % 2 == 1;
    println!("{}", if yes { "Takahashi" } else { "Aoki" });
}
