use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        t: usize,
        a: [usize; n],
    }

    let result = a.iter().filter(|&&x| x.abs_diff(s) <= t).count();
    println!("{result}");
}
