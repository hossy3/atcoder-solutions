use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let v = vec![
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
    ];
    for y in 0..h {
        for x in 0..w {
            if s[y][x] != 's' {
                continue;
            }
            for &(dx, dy) in &v {
                let x4 = x as i64 + dx * 4;
                if x4 < 0 || x4 >= w as i64 {
                    continue;
                }
                let y4 = y as i64 + dy * 4;
                if y4 < 0 || y4 >= h as i64 {
                    continue;
                }
                let x1 = (x as i64 + dx) as usize;
                let y1 = (y as i64 + dy) as usize;
                let x2 = (x as i64 + dx * 2) as usize;
                let y2 = (y as i64 + dy * 2) as usize;
                let x3 = (x as i64 + dx * 3) as usize;
                let y3 = (y as i64 + dy * 3) as usize;
                let x4 = x4 as usize;
                let y4 = y4 as usize;
                if s[y1][x1] == 'n' && s[y2][x2] == 'u' && s[y3][x3] == 'k' && s[y4][x4] == 'e' {
                    println!("{} {}", y + 1, x + 1);
                    println!("{} {}", y1 + 1, x1 + 1);
                    println!("{} {}", y2 + 1, x2 + 1);
                    println!("{} {}", y3 + 1, x3 + 1);
                    println!("{} {}", y4 + 1, x4 + 1);
                    return;
                }
            }
        }
    }
}
