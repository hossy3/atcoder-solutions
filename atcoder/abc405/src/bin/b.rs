use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; n],
    }

    let mut v = vec![0usize; m];
    for &x in &a {
        v[x] += 1;
    }

    let mut result = 0usize;
    if v.iter().all(|&x| x > 0) {
        for &x in a.iter().rev() {
            result += 1;
            v[x] -= 1;
            if v[x] == 0 {
                break;
            }
        }
    }
    println!("{result}");
}
