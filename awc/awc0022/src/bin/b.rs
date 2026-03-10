use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
        a: [usize; n],
    }

    let result = a.iter().map(|&x| t.saturating_sub(x)).sum::<usize>();
    if result <= m {
        println!("{result}");
    } else {
        println!("-1");
    }
}
