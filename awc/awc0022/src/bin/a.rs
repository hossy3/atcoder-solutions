use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        k: usize,
        w: [usize; n],
    }

    let result = w.iter().filter(|&&x| x > d * k).count();
    println!("{result}");
}
