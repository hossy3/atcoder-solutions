use std::collections::VecDeque;

use proconio::{input, marker::{Chars, Usize1}};

fn main() {
    input! {
        n: usize,
        ax: Usize1,
        ay: Usize1,
        bx: Usize1,
        by: Usize1,
        s: [Chars; n],
    }

    if (ax + ay + bx + by) % 2 == 1 {
        println!("{}", -1);
        return;
    } 

    let mut v = vec![vec![std::usize::MAX; n]; n];
    for x in 0..n {
        for y in 0..n {
            if s[x][y] == '#' {
                v[x][y] = 0;
            }
        }
    }
    let mut query = VecDeque::new();
    v[ax][ay] = 0;
    query.push_back(((ax, ay), true));
    query.push_back(((ax, ay), false));
    while let Some(((x, y), dir)) = query.pop_front() {
        if x == bx && y == by {
            println!("{}", v[x][y]);
            return;
        }
        let step = v[x][y] + 1;

        if dir {
            for i in 1..=(x.min(y)) {
                let (x0, y0) = (x - i, y - i);
                if v[x0][y0] < step {
                    break;
                }
                if v[x0][y0] > step {
                    v[x0][y0] = step;
                    query.push_back(((x0, y0), !dir));
                }
            }

            for i in 1..=((n - x - 1).min(n - y - 1)) {
                let (x0, y0) = (x + i, y + i);
                if v[x0][y0] < step {
                    break;
                }
                if v[x0][y0] > step {
                    v[x0][y0] = step;
                    query.push_back(((x0, y0), !dir));
                }
            }
        }
        
        if !dir {
            for i in 1..=(x.min(n - y - 1)) {
                let (x0, y0) = (x - i, y + i);
                if v[x0][y0] < step {
                    break;
                }
                if v[x0][y0] > step {
                    v[x0][y0] = step;
                    query.push_back(((x0, y0), !dir));
                }
            }
    
            for i in 1..=((n - x - 1).min(y)) {
                let (x0, y0) = (x + i, y - i);
                if v[x0][y0] < step {
                    break;
                }
                if v[x0][y0] > step {
                    v[x0][y0] = step;
                    query.push_back(((x0, y0), !dir));
                }
            }
        }
    }

    println!("{}", -1);
}
