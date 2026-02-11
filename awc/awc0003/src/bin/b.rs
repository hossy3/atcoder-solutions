use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(char, char); n],
    }
    let result = (0..(n - 1)).filter(|&i| lr[i].1 == lr[i + 1].0).count();
    println!("{result}");
}
