use std::collections::BTreeMap;

use proconio::input;

fn f(m: usize, c: usize, a: &[usize]) -> usize {
    // それぞれの位置に何人立っているか
    let mut map0 = BTreeMap::new();
    for &x in a {
        *map0.entry((x + m - 1) % m).or_insert(0usize) += 1;
    }
    let mut v0 = vec![];
    for (&x, &count) in &map0 {
        v0.push((x, count));
    }

    // しゃくとり法 (two-pointer) で余りを処理する
    let mut v1 = vec![];

    let mut r = 0usize;
    let mut count = 0usize;
    for l in 0..(v0.len())  {
        while count < c {
            count += v0[r].1;
            r = (r + 1) % v0.len();
        }
        v1.push((v0[l].0, count));
        count -= v0[l].1;
    }

    let pair = (m - 1, v1[0].1);
    v1.push(pair);

    let mut cur = -1;
    let mut result = 0;
    for &(x0, x1) in &v1 {
        result += (x0 as isize - cur) as usize * x1;
        cur = x0 as isize;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(f(3, 2, &[1, 2, 1, 0, 1]), 9);
        assert_eq!(f(4, 2, &[1, 2, 1, 0, 1]), 13);
        assert_eq!(f(5, 2, &[1, 2, 1, 0, 1]), 17);
        assert_eq!(f(6, 2, &[1, 2, 1, 0, 1]), 21);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        c: usize,
        a: [usize; n],
    }

    let result = f(m, c, &a);
    println!("{result}");
}
