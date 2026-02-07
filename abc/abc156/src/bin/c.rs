use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [isize; n],
    }
    let result = (1..=100)
        .map(|p| x.iter().map(|&x| (x - p).pow(2)).sum::<isize>())
        .min()
        .unwrap();
    println!("{result}");
}
