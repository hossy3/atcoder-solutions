use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut result = 0;
    for _ in 0..n {
        input! {
            a: usize,
            b: [usize; a],
        }
        result += b.iter().filter(|&&x| x >= k).count();
    }

    println!("{result}");
}
