use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut c: [usize; n],
        t: [Usize1; m],
    }

    let mut result = 0;
    for &t in &t {
        if c[t] > 0 {
            c[t] -= 1;
            result += 1;
        }
    }
    println!("{result}");
}
