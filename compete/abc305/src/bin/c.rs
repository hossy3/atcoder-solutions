use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut i0 = std::usize::MAX;
    let mut i1 = 0;
    let mut j0 = std::usize::MAX;
    let mut j1 = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                i0 = i0.min(i);
                i1 = i1.max(i);
                j0 = j0.min(j);
                j1 = j1.max(j);
            }
        }
    }

    for i in i0..=i1 {
        for j in j0..=j1 {
            if s[i][j] != '#' {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }

    panic!();
}
