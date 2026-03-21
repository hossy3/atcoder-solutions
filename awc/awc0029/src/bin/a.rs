use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        b: usize,
        k: usize,
        c: [usize; n],
    }

    let result = c
        .iter()
        .map(|&x| x * if x >= k { p + b } else { p })
        .sum::<usize>();
    println!("{result}");
}
