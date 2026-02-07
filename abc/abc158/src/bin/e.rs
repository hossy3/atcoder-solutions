use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        p: usize,
        s: Chars,
    }

    fn f(c: char) -> usize {
        c as usize - '0' as usize
    }

    // 2 と 5 は 10 の倍数で以下の処理が効かないので、別扱いする
    if p == 2 || p == 5 {
        let mut result = 0usize;
        for (i, &c) in s.iter().enumerate() {
            if f(c) % p == 0 {
                result += i + 1; // 上の桁はいくつでも良い
            }
        }
        println!("{result}");
        return;
    }

    let mut m = vec![1usize]; // [1 % p, 10 % p, 100 % p, ...]
    for _ in 1..n {
        let k = (*m.last().unwrap() * 10) % p;
        m.push(k);
    }

    // p で割った余りの個数を各桁について調べる
    // たとえば s=3543 の場合 [3000 % p, 3500 % p, 3540 % p, 3543 % p] を得る
    let mut x = 0;
    let mut v = vec![];
    for (i, &c) in s.iter().enumerate() {
        x = (x + f(c) * m[n - i - 1]) % p;
        v.push(x);
    }

    // それぞれの余りの数を数え、組み合わせる
    // たとえば 3000 % p と 3540 % p が同じなら、引いた 540 % p は余りなしになるはず
    // また、3000 % p が余りなしなら単独でも答えに加える
    let mut map = HashMap::new();
    for &x in &v {
        *map.entry(x).or_insert(0usize) += 1;
    }

    let result = map.get(&0).unwrap_or(&0)
        + map
            .iter()
            .map(|(_, &count)| count * (count - 1) / 2)
            .sum::<usize>();
    println!("{result}");
}
