use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut v = vec![vec![]; n];
    for (i, &x) in a.iter().enumerate() {
        v[x].push(i);
    }

    let mut result = 0;
    for v in &v {
        if v.len() == 0 {
            continue;
        }
        result += (n + 1) * n / 2;
        for v in v.windows(2) {
            let (j0, j1) = (v[0], v[1]);
            result -= (j1 - j0) * (j1 - j0 - 1) / 2;
        }
        let j = v[0];
        result -= j * (j + 1) / 2;        
        let j = v[v.len() - 1];
        result -= (n - j) * (n - j - 1) / 2;        
    }

    println!("{result}");
}
