use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, char); m],
    }

    let mut v = vec![true; n];

    for (a, b) in ab {
        if v[a] && b == 'M' {
            println!("Yes");
            v[a] = false;
        } else {
            println!("No");
        }
    }
}
