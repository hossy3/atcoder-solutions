use proconio::{input, marker::Chars};

fn f(t: (char, char, char, char, char, char)) -> usize {
    let x = if t.0 == '#' { 1 } else { 0 }
        + if t.1 == '#' { 1 } else { 0 }
        + if t.2 == '#' { 1 } else { 0 }
        + if t.3 == '#' { 1 } else { 0 }
        + if t.4 == '#' { 1 } else { 0 }
        + if t.5 == '#' { 1 } else { 0 };
    x
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let g = || {
        for i in 0..n {
            for j in 0..(n - 5) {
                if f((
                    s[i][j],
                    s[i][j + 1],
                    s[i][j + 2],
                    s[i][j + 3],
                    s[i][j + 4],
                    s[i][j + 5],
                )) >= 4
                {
                    return true;
                }
            }
        }

        for i in 0..(n - 5) {
            for j in 0..n {
                if f((
                    s[i][j],
                    s[i + 1][j],
                    s[i + 2][j],
                    s[i + 3][j],
                    s[i + 4][j],
                    s[i + 5][j],
                )) >= 4
                {
                    return true;
                }
            }
        }

        for i in 0..(n - 5) {
            for j in 0..(n - 5) {
                if f((
                    s[i][j],
                    s[i + 1][j + 1],
                    s[i + 2][j + 2],
                    s[i + 3][j + 3],
                    s[i + 4][j + 4],
                    s[i + 5][j + 5],
                )) >= 4
                {
                    return true;
                }

                if f((
                    s[i][j + 5],
                    s[i + 1][j + 4],
                    s[i + 2][j + 3],
                    s[i + 3][j + 2],
                    s[i + 4][j + 1],
                    s[i + 5][j],
                )) >= 4
                {
                    return true;
                }
            }
        }

        false
    };

    let yes = g();
    println!("{}", if yes { "Yes" } else { "No" });
}
