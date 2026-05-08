use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let result = a.iter().sum::<usize>() + (n - 1) / k;
    println!("{result}");
}
