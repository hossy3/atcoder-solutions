use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n - 1],
    }

    let mut result = 1;
    let mut cur = 0;
    while cur < n - 1 {
        cur = p[cur];
        result += 1;
    }
    println!("{result}");
}
