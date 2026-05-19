use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let result = a.iter().fold(m, |acc, &x| acc * x / 100);
    println!("{result}");
}
