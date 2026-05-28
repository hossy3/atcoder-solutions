use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        c: [usize; m],
    }

    let sum = c.iter().map(|&c| c.min(n)).sum::<usize>();
    let yes = sum >= k * n;
    println!("{}", if yes { "Yes" } else { "No" });
}
