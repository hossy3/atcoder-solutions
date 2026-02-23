use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    }

    h.sort();
    let result = h[..(n - k)].iter().sum::<usize>() + k;
    println!("{result}");
}
