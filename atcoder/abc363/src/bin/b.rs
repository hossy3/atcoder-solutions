use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        p: usize,
        l: [usize; n],
    }

    let result = (0..=100)
        .find(|i| l.iter().filter(|&&j| i + j >= t).count() >= p)
        .unwrap();
    println!("{result}");
}
