use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 9],
    }
    let mut count = 0;
    for x0 in 0..9 {
        for y0 in 0..9 {
            for x1 in (x0 + 1)..9 {
                for y1 in y0..9 {
                    let x2 = x1 - (y1 - y0);
                    let y2 = y1 + (x1 - x0);
                    let x3 = x0 - (y1 - y0);
                    let y3 = y0 + (x1 - x0);
                    if y2 >= 9 || x3 < 0 {
                        continue;
                    }
                    if s[y0 as usize][x0 as usize] == '#'
                        && s[y1 as usize][x1 as usize] == '#'
                        && s[y2 as usize][x2 as usize] == '#'
                        && s[y3 as usize][x3 as usize] == '#'
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("{}", count);
}
