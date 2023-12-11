use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        k: usize,
        pq: [(usize, usize); n],
    }
    let mut result = pq.iter().fold(0, |acc, (p, q)| acc + p * q);
    if result < s {
        result += k;
    }
    println!("{result}");
}
