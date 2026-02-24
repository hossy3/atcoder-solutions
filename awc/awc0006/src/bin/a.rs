use proconio::input;

fn main() {
    input! {
        n: usize,
        l: isize,
        w: isize,
        a: [isize; n],
    }
    let result = a.iter().filter(|&&x| l - w <= x && x <= l + w).count();
    println!("{result}");
}
