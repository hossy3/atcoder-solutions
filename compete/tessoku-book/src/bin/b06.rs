use proconio::{input, marker::Usize1};

fn main() {
    input! {
        a: [usize],
        lr: [(Usize1, Usize1)],
    }

    let mut cum = vec![Vec::with_capacity(a.len() + 1); 2];
    cum[0].push(0);
    cum[1].push(0);
    for &a in &a {
        let b = *cum[a].last().unwrap() + 1;
        let c = *cum[1 - a].last().unwrap();
        cum[a].push(b);
        cum[1 - a].push(c);
    }

    for &(l, r) in &lr {
        let n0 = cum[0][r + 1] - cum[0][l];
        let n1 = cum[1][r + 1] - cum[1][l];
        let result = if n1 > n0 {
            "win"
        } else if n1 < n0 {
            "lose"
        } else {
            "draw"
        };
        println!("{}", result);
    }
}
