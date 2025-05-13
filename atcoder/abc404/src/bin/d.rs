use proconio::{input, marker::Usize1};

fn f(b: &[Vec<usize>], first: bool, c: &[usize], cur: Vec<usize>, money: usize) -> usize {
    if cur.iter().all(|&x| x >= 2) {
        return money;
    }

    let mut result = usize::MAX;
    for i in 0..b.len() {
        let mut cur = cur.clone();
        let money = money + c[i];
        for &j in &b[i] {
            cur[j] += 1;
        }
        if i == 0 {
            if first {
                result = result.min(f(b, false, c, cur, money));
            } else {
                result = result.min(f(&b[1..], true, &c[1..], cur, money));
            }
        } else {
            result = result.min(f(&b[i..], false, &c[i..], cur, money));
        }
    }

    result
}

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [usize; n],
    }

    let mut a = vec![]; // 動物 to 動物園 
    let mut b = vec![vec![]; n]; // 動物園 to 動物
    for i in 0..m {
        input! {
            k: usize,
            a0: [Usize1; k],
        }
        for &x in &a0 {
            b[x].push(i); // 動物園 x に行くと動物 i が見られる
        }
        a.push(a0);
    }

    let cur = vec![0usize; m];
    let result = f(&b, true, &c, cur, 0);
    println!("{result}");
}
