use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        ac: [(usize, usize); n],
    }

    let result = ac
        .iter()
        .map(|&(a, c)| t.saturating_sub(a) * c)
        .sum::<usize>();
    println!("{result}");
}
