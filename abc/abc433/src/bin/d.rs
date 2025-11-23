use std::collections::HashMap;

use proconio::input;

// 桁数
fn f(i: usize) -> usize {
    i.ilog10() as usize + 1
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut v = vec![HashMap::new(); 11]; // 最大10桁
    for &x in &a {
        let k = f(x);
        *v[k].entry(x % m).or_insert(0_usize) += 1;
    }

    let mut result = 0;
    for &x in &a {
        for k in 1..=10 {
            let y = (x * 10usize.pow(k as u32)) % m;
            let y = (m - y) % m;
            if let Some(i) = v[k].get(&y) {
                result += i;
            }
        }
    }

    println!("{result}");
}
