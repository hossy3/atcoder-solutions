use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }
    let diff = a.iter().zip(b.iter()).fold(0, |acc, x| acc + (x.0 - x.1).abs());
    let yes = (k >= diff) && ((k - diff) % 2 == 0);
    println!("{}", if yes { "Yes" } else { "No" });
}
