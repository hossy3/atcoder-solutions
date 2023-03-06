use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut v = vec![0; n];
    for _ in 0..q {
        input! {
            t: usize,
            x: Usize1,
        }
        match t {
            1 => {
                v[x] += 1;
            }
            2 => {
                v[x] += 2;
            }
            _ => {
                let yes = v[x] >= 2;
                println!("{}", if yes { "Yes" } else { "No" });
            }
        }
    }
}
