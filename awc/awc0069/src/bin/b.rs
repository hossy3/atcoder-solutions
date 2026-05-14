use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        tc: [(isize, isize); n],
    }
    let mut tc = tc.iter().map(|&(a, b)| a + b).collect::<Vec<_>>();
    tc.sort_unstable();
    tc.reverse();
    tc.truncate(k);
    let result = tc.iter().sum::<isize>();
    println!("{result}");
}
