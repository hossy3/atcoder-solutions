use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut i: Usize1,
        mut j: Usize1,
        c: [Chars; h],
        x: Chars,
    }
    for x in x {
        match x {
            'L' => {
                if j > 0 && c[i][j - 1] == '.' {
                    j -= 1
                };
            }
            'R' => {
                if j + 1 < w && c[i][j + 1] == '.' {
                    j += 1
                };
            }
            'U' => {
                if i > 0 && c[i - 1][j] == '.' {
                    i -= 1
                };
            }
            'D' => {
                if i + 1 < h && c[i + 1][j] == '.' {
                    i += 1
                };
            }
            _ => unreachable!(),
        }
    }
    println!("{} {}", i + 1, j + 1);
}
