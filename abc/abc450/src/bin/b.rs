use proconio::input;

fn f(c: &[Vec<usize>]) -> bool {
    let n = c.len() + 1;
    for i in 0..(n - 2) {
        for j in (i + 1)..(n - 1) {
            for k in (j + 1)..n {
                // eprintln!("{} {} {} {} {} {}", n, i, j, k, j - i - 1, k - i - 1);
                if c[i][j - i - 1] + c[j][k - j - 1] < c[i][k - i - 1] {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
    }

    let mut c = vec![];
    for i in 0..(n - 1) {
        input! {
            c0: [usize; n - i - 1],
        }
        c.push(c0);
    }
    let yes = f(&c);
    println!("{}", if yes { "Yes" } else { "No" });
}
