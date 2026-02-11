use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        m: isize,
        a: [isize; n - 1],
    }
    let result = (m * n as isize - a.iter().sum::<isize>()).max(0);
    if result > k {
        println!("-1");
    } else {
        println!("{result}");
    }
}
