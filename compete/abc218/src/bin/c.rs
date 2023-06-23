use proconio::{input, marker::Chars};

fn f(s: &[Vec<char>], t: usize) -> Vec<(usize, usize)> {
    let n = s.len();
    let mut v = vec![];
    for y in 0..n {
        for x in 0..n {
            if s[y][x] == '#' {
                let pair = match t {
                    0 => (x, y),
                    1 => (n - y - 1, x),
                    2 => (n - x - 1, n - y - 1),
                    _ => (y, n - x - 1),
                };
                v.push(pair);
            }
        }
    }

    let x0 = v.iter().min_by_key(|(x, _)| x).unwrap().0;
    let y0 = v.iter().min_by_key(|(_, y)| y).unwrap().1;
    for i in 0..v.len() {
        v[i] = (v[i].0 - x0, v[i].1 - y0);
    }
    v.sort();
    v
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    }
    let s0 = f(&s, 0);
    let yes = (0..4).any(|i| f(&t, i) == s0);
    println!("{}", if yes { "Yes" } else { "No" });
}
