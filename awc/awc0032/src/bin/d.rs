use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut d: [usize; m],
    }

    a.sort_unstable();
    d.sort_unstable();
    a.reverse();
    d.reverse();

    let mut result = 0;
    for (&a, &d) in std::iter::zip(&a, &d) {
        if a < d {
            println!("-1");
            return;
        }
        result += a - d;
    }
    println!("{result}");
}
