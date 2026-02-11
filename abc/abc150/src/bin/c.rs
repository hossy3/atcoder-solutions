use itertools::Itertools;
use proconio::input;

fn f(p: &[usize]) -> usize {
    let n = p.len();
    let mut i = 0;
    for v in (1..=n).permutations(n) {
        if v == *p {
            return i;
        }
        i += 1;
    }
    unreachable!()
}

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }
    let result = f(&p).abs_diff(f(&q));
    println!("{result}");
}
