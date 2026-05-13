use proconio::input;

fn main() {
    input! {
        n: usize,
        l: isize,
        r: isize,
        t: [isize; n],
    }

    let result = t.iter().filter(|&&t| l <= t && t <= r).count();
    println!("{result}");
}
