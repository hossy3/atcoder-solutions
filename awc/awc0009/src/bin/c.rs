use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        k: usize,
        h: [usize; n],
    }

    let d = *h.iter().min().unwrap() - 1;
    let result = h.iter().filter(|&x| x - d <= t + k).count();
    println!("{result}");
}
