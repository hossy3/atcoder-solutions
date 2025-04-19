use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut itoa = vec![]; // 料理から食材
    let mut atoi = vec![vec![]; n]; // 食材から料理
    let mut itor = vec![]; // 料理から残った食材の苦手数
    for i in 0..m {
        input! {
            k: usize,
            a: [Usize1; k],
        }
        itor.push(a.len());
        for &a in &a {
            atoi[a].push(i);
        }
        itoa.push(a);
    }

    input! {
        b: [Usize1; n],
    }

    let mut result = 0usize;
    for &b in &b {
        for &i in &atoi[b] {
            itor[i] -= 1;
            if itor[i] == 0 {
                result += 1;
            }
        }
        println!("{result}");
    }
}
