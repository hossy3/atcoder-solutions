use proconio::{input, marker::Chars};

fn is_same(i0: usize, i1: usize) -> bool {
    i0 == i1 && i0 != 0
}

fn is_palindrome(s: &[usize]) -> bool {
    let k = s.len();
    (0..(k / 2)).all(|i| is_same(s[i], s[k - i - 1]))
}

fn f(k: usize, s: &Vec<char>) -> usize {
    let n = s.len();

    // 文字列を 1回だけ現れるものをすべて 0 に、それ以外を 1 から始まる順に組み立て直す
    let mut a = [0usize; 26];
    for &c in s {
        let i = c as usize - 'a' as usize;
        a[i] += 1;
    }

    let mut counts = vec![0usize; n];
    for k in a {
        if k > 0 {
            counts[k - 1] += 1;
        }
    }

    let mut cur = 1usize;
    let mut s = vec![0usize; counts[0]];
    for (i, &k) in counts[1..].iter().enumerate() {
        for _ in 0..k {
            s.append(&mut vec![cur; i + 2]);
            cur += 1;
        }
    }
    let ref s = s;

    let mut result = 0usize;
    let mut stack: Vec<(_, Vec<_>)> = vec![(0, (*s).clone())]; // (選択済み数, 現在の配列)
    while let Some((i, s)) = stack.pop() {
        let mut set = [false; 6]; // aabbccddee
        let palindrome_candidate = i + 1 >= k && is_palindrome(&s[(i + 2 - k)..i]);
        for j in i..n {
            let key = s[j];
            if set[key] {
                continue;
            }
            set[key] = true;
            let mut s = s.clone();
            s.swap(i, j);
            if palindrome_candidate && i + 1 >= k && is_same(s[i], s[i + 1 - k]) {
                continue;
            }
            if i + 1 == n {
                result += 1;
            } else {
                stack.push((i + 1, s));
            }
        }
    }

    // 1回だけ現れる文字はどこに現れても良いので、階乗をかける
    result *= (1..=counts[0]).fold(1, |acc, x| acc * x);
    result
}

fn main() {
    input! {
        _: usize,
        k: usize,
        s: Chars,
    }

    let result = f(k, &s);
    println!("{result}");
}
