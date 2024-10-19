use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        ht: [(char, Usize1); q],
    }
    let mut result = 0;
    let mut l = 0;
    let mut r = 1;
    for (x, t) in ht {
        if x == 'L' {
            if l < r {
                if t < r {
                    result += l.abs_diff(t);
                } else {
                    result += (l + n).abs_diff(t);
                }
            } else {
                if r < t {
                    result += l.abs_diff(t);
                } else {
                    result += l.abs_diff(t + n);
                }
            }
            l = t;
        } else {
            if l < r {
                if t < l {
                    result += r.abs_diff(t + n);
                } else {
                    result += r.abs_diff(t);
                }
            } else {
                if l < t {
                    result += (r + n).abs_diff(t);
                } else {
                    result += r.abs_diff(t);
                }
            }
            r = t;
        }
    }
    println!("{result}");
}
