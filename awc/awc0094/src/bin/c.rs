use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [usize; n],
    }
    let max = x.iter().max().unwrap();
    let min = x.iter().min().unwrap();
    let result = (max - min) * 2;
    println!("{result}");
}
