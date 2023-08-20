use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..s {
            if !dp[i][j] {
                continue;
            }
            let (a, b) = ab[i];
            if j + a <= s {
                dp[i + 1][j + a] = true;
            }
            if j + b <= s {
                dp[i + 1][j + b] = true;
            }
        }
    }
     
    if !dp[n][s] {
        println!("Impossible");
        return;
    }

    let mut course = vec!['A'; n];
    let mut j = s;
    for i in (0..n).rev() {
        let (a, b) = ab[i];
        if j >= a && dp[i][j - a] {
            j -= a;
        } else {
            j -= b;
            course[i] = 'B';
        }
    }
    let course: String = course.into_iter().collect();
    println!("{}", course);
}
