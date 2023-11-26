use proconio::{input, marker::Chars};

fn to_i(c: char) -> usize {
    c as usize - 'a' as usize
}

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let mut v = [0; 26];
    let mut cur = (to_i(s[0]), 1);
    v[cur.0] = cur.1;

    for &c in &s[1..] {
        let i = to_i(c);
        if i == cur.0 {
            cur.1 += 1;
        } else {
            cur = (i, 1);
        }
        v[cur.0] = v[cur.0].max(cur.1);
    }

    let result: usize = v.iter().sum();
    println!("{result}");
}
