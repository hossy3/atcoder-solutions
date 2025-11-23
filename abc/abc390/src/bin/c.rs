use proconio::{input, marker::Chars};

fn f(s: &[Vec<char>]) -> bool {
    let h = s.len();
    let w = s[0].len();

    let mut h_min = usize::MAX;
    let mut h_max = usize::MIN;
    let mut w_min = usize::MAX;
    let mut w_max = usize::MIN;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                h_min = h_min.min(i);
                h_max = h_max.max(i);
                w_min = w_min.min(j);
                w_max = w_max.max(j);
            }
        }
    }

    for i in h_min..=h_max {
        for j in w_min..=w_max {
            if s[i][j] == '.' {
                return false;
            }
        }
    }

    true
}

fn main() {
    input! {
        h: usize,
        _: usize,
        s: [Chars; h],
    }
    let yes = f(&s);
    println!("{}", if yes { "Yes" } else { "No" });
    // let result = n;
    // println!("{result}");
}
