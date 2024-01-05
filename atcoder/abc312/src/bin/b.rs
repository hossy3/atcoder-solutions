use proconio::{input, marker::Chars};

fn f(s: &[Vec<char>], i: usize, j: usize) -> bool {
    for i0 in 0..3 {
        for j0 in 0..3 {
            if s[i + i0][j + j0] != '#' {
                return false;
            }
        }
    }

    for i0 in 6..9 {
        for j0 in 6..9 {
            if s[i + i0][j + j0] != '#' {
                return false;
            }
        }
    }

    for i0 in 0..3 {
        if s[i + i0][j + 3] != '.' {
            return false;
        }
    }

    for i0 in 6..9 {
        if s[i + i0][j + 5] != '.' {
            return false;
        }
    }

    for j0 in 0..3 {
        if s[i + 3][j + j0] != '.' {
            return false;
        }
    }

    for j0 in 6..9 {
        if s[i + 5][j + j0] != '.' {
            return false;
        }
    }

    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    for i in 0..=(n - 9) {
        for j in 0..=(m - 9) {
            if f(&s, i, j) {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
}
