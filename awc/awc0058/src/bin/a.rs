use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        t: usize,
        c: isize,
        s: [usize; n - 1],
    }
    let s_max = s.iter().max().copied().unwrap_or(p);
    let result = if p >= t {
        0
    } else if s_max >= t {
        c
    } else {
        -1
    };
    println!("{result}");
}
