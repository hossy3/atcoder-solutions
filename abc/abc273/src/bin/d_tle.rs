use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        rs: usize,
        cs: usize,
        n: usize,
        rc: [(usize, usize); n],
        q: usize,
        dl: [(char, usize); q],
    }
    
    let mut s = HashSet::new();
    for &x in &rc {
        s.insert(x);
    }

    let mut r = rs;
    let mut c = cs;
    for &(d, l) in &dl {
        for _ in 0..l {
            let mut r0 = r;
            let mut c0 = c;
            match d {
                'U' => {
                    r0 -= 1;
                    if r0 == 0 {
                        break;
                    }
                }
                'D' => {
                    r0 += 1;
                    if r0 == h + 1 {
                        break;
                    }
                }
                'L' => {
                    c0 -= 1;
                    if c0 == 0 {
                        break;
                    }
                }
                'R' => {
                    c0 += 1;
                    if c0 == w + 1 {
                        break;
                    }
                }
                _ => {}
            }
            if s.contains(&(r0, c0)) {
                break;
            }
            r = r0;
            c = c0;
        }

        println!("{} {}", r, c);
    }
}
