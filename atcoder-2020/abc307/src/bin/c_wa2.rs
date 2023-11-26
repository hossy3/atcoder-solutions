use proconio::{input, marker::Chars};

fn cut(a: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut y0 = a.len();
    let mut y1 = 0;
    let mut x0 = a[0].len();
    let mut x1 = 0;
    for y in 0..(a.len()) {
        for x in 0..(a[0].len()) {
            if a[y][x] == '#' {
                y0 = y0.min(y);
                y1 = y1.max(y + 1);
                x0 = y0.min(x);
                x1 = y1.max(x + 1);
            }
        }
    }
    let mut v = vec![vec!['.'; x1 - x0]; y1 - y0];
    for y in y0..y1 {
        for x in x0..x1 {
            v[y - y0][x - x0] = a[y][x];
        }
    }
    v
}

fn f(a: &[Vec<char>], y: i64, x: i64) -> char {
    if y < 0 || a.len() as i64 <= y || x < 0 || a[0].len() as i64 <= x {
        '.'
    } else {
        a[y as usize][x as usize]
    }
}

fn main() {
    input! {
        ha: usize,
        _: usize,
        a: [Chars; ha],
        hb: usize,
        _: usize,
        b: [Chars; hb],
        hx: usize,
        wx: usize,
        x: [Chars; hx],
    }

    let a = cut(&a);
    let ha = a.len();
    let wa = a[0].len();
    let b = cut(&b);
    let hb = b.len();
    let wb = b[0].len();
    if ha > hx || wa > wx || hb > hx || wb > wx {
        println!("No");
        return;
    }

    for a0 in 0..=(hx - ha) {
        for a1 in 0..=(wx - wa) {
            for b0 in 0..=(hx - hb) {
                'outer: for b1 in 0..=(wx - wb) {
                    for c0 in 0..hx {
                        for c1 in 0..wx {
                            if (x[c0][c1] == '.'
                                && (f(&a, c0 as i64 - a0 as i64, c1 as i64 - a1 as i64) == '#'
                                    || f(&b, c0 as i64 - b0 as i64, c1 as i64 - b1 as i64) == '#'))
                                || (x[c0][c1] == '#'
                                    && (f(&a, c0 as i64 - a0 as i64, c1 as i64 - a1 as i64) == '.'
                                        && f(&b, c0 as i64 - b0 as i64, c1 as i64 - b1 as i64)
                                            == '.'))
                            {
                                continue 'outer;
                            }
                        }
                    }
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
