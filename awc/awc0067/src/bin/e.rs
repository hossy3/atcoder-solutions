use proconio::input;

/// Nim ゲームで先手が勝つかを返す
fn nim_winner(k: usize, a: &[usize]) -> bool {
    fn grundy(k: usize, a: usize) -> usize {
        a % (k + 1)
    }
    let result = a.iter().fold(0, |acc, &a| acc ^ grundy(k, a));
    result != 0
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let yes = nim_winner(k, &a);
    println!("{}", if yes { "Takahashi" } else { "Aoki" });
}
