use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [isize; n],
    }
    x.sort_unstable();
    let x0 = x[x.len() / 2];
    let result = x.iter().map(|&x| (x - x0).abs()).sum::<isize>();
    println!("{result}");
}
