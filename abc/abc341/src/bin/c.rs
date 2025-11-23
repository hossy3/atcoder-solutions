use proconio::{input, marker::Chars};

fn f((mut r, mut c): (usize, usize), t: &[char], s: &[Vec<char>]) -> bool {
    let h = s.len();
    let w = s[0].len();

    if s[r][c] == '#' {
        return false;
    }
    for &t in t {
        (r, c) = match t {
            'L' => (r, c.wrapping_sub(1)),
            'R' => (r, c.wrapping_add(1)),
            'U' => (r.wrapping_sub(1), c),
            'D' => (r.wrapping_add(1), c),
            _ => unreachable!(),
        };
        if r >= h || c >= w || s[r][c] == '#' {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        (h, w, _): (usize, usize, usize),
        t: Chars,
        s: [Chars; h],
    }

    let mut count = 0usize;
    for r in 0..h {
        for c in 0..w {
            if f((r, c), &t, &s) {
                count += 1;
            }
        }
    }
    println!("{count}");
}
