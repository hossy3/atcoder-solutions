use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }

    let result = p.iter().filter(|&&x| x >= k).sum::<usize>();
    println!("{result}");
}
