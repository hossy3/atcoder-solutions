use proconio::{input, marker::Chars};

#[allow(dead_code)]
fn combination(n: usize, r: usize) -> usize {
    if n < r || n == 0 {
        return 0;
    }
    let r = r.min(n - r);
    (1..=r).fold(1, |acc, x| acc * (n - x + 1) / x)
}

#[allow(dead_code)]
fn f_another(n: &[char], k: usize) -> usize {
    // 0 ではない桁を左から k 個調べる
    let mut v = vec![];
    for (i, &c) in n.iter().enumerate() {
        let x = c as usize - '0' as usize;
        if x > 0 {
            v.push((i, x));
            if v.len() == k {
                break;
            }
        }
    }

    let mut result = 0usize;
    for (j, &(i, x)) in v.iter().enumerate() {
        let m = n.len() - i - 1; // 下の桁の数

        // その桁に 0 を埋めると、下の桁のうち k-j 個はなんでも良い。ただし数が十分あること
        result += combination(m, k - j) * 9usize.pow((k - j) as u32);

        if j < k - 1 {
            // 最後以外の桁に 0 でも最大値でもない値を埋めると、下の桁のうち k-j-1 個はなんでも良い
            result += (x - 1) * combination(m, k - j - 1) * 9usize.pow((k - j - 1) as u32);
        } else {
            // 最後の桁に 0 以外の値を埋めると、下の桁はすべて 0
            result += x;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        for i in 1..=10000 {
            for k in 1..=3 {
                let n = i.to_string().chars().collect::<Vec<char>>();
                assert_eq!(f(&n, k), f_another(&n, k), "{i}, {k}");
            }
        }
    }
}

#[allow(dead_code)]
fn f(n: &[char], k: usize) -> usize {
    let mut v = vec![0usize; k + 1]; // [0以外を使った回数] = 何通り ただし最大の数は含めない
    let mut k0 = 0; // 最大の数では 0 を何回使ったか

    for &c in n {
        let mut v0 = v.clone(); // 次の桁に 0 を入れる通り数は前回と同じ

        let x = c as usize - '0' as usize;
        for j in 0..k {
            v0[j + 1] += v[j] * 9; // 次の桁に 1..=9 を入れる
        }
        if x > 0 && k0 < k {
            // 3xyz のような4桁の数字のとき
            v0[k0] += 1; // 0xyz だと xyz は自由
            v0[k0 + 1] += x - 1; // 1xyz, 2xyz だと xyz は自由
            k0 += 1; // 3を確定させると x は x 以下しか選べない
            if k0 == k {
                v0[k] += 1; // ここまで最大の数で確定すると、残りは 0 のみ選べる
            }
        }

        v = v0;
    }

    v[k]
}

fn main() {
    input! {
        n: Chars,
        k: usize,
    }

    let result = f_another(&n, k);
    println!("{result}");
}
