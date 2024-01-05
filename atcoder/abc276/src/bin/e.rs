use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut sy = 0;
    let mut sx = 0;

    'outer: for y in 0..h {
        for x in 0..w {
            if c[y][x] == 'S' {
                sy = y;
                sx = x;
                break 'outer;
            }
        }
    }

    for k in 0..4 {
        let mut cy = sy;
        let mut cx = sx;
        if k == 0 {
            if cy == 0 {
                continue;
            }
            cy -= 1;
        }
        if k == 1 {
            if cy == h - 1 {
                continue;
            }
            cy += 1;
        }
        if k == 2 {
            if cx == 0 {
                continue;
            }
            cx -= 1;
        }
        if k == 3 {
            if cx == w - 1 {
                continue;
            }
            cx += 1;
        }
        if c[cy][cx] != '.' {
            continue;
        }

        let mut m = vec![vec![false; w]; h];
        let mut stack = Vec::new();
        stack.push((cx, cy));
        while let Some((x, y)) = stack.pop() {
            m[y][x] = true;
            let mut v = Vec::new();
            if y > 0 {
                v.push((x, y - 1));
            }
            if y < h - 1 {
                v.push((x, y + 1));
            }
            if x > 0 {
                v.push((x - 1, y));
            }
            if x < w - 1 {
                v.push((x + 1, y));
            }
            for &(x, y) in &v {
                if c[y][x] == '.' && !m[y][x] {
                    stack.push((x, y));
                }
            }
        }

        let mut count = 0;
        if (sy > 0) && m[sy - 1][sx] {
            count += 1;
        }
        if (sy < h - 1) && m[sy + 1][sx] {
            count += 1;
        }
        if (sx > 0) && m[sy][sx - 1] {
            count += 1;
        }
        if (sx < w - 1) && m[sy][sx + 1] {
            count += 1;
        }
        if count > 1 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
