use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut result = 0usize;
    for i0 in 0..n {
        for j0 in 0..n {
            if s[i0][j0] != 'o' {
                continue;
            }

            let mut count0 = 0;
            for i1 in 0..n {
                if i1 != i0 && s[i1][j0] == 'o' {
                    count0 += 1;
                }
            }

            let mut count1 = 0;
            for j1 in 0..n {
                if j1 != j0 && s[i0][j1] == 'o' {
                    count1 += 1;
                }
            }

            result += count0 * count1;
        }
    }
    println!("{result}");
}
