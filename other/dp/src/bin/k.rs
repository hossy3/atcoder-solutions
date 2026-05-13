use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut dp = vec![true; k + 1]; // 自分の手版になったとき勝てるか
    'outer: for i in 0..=k {
        for &a in &a {
            if a > i {
                break;
            }
            let j = i - a;
            if dp[j] == false {
                dp[i] = true;
                continue 'outer;
            }
        }
        dp[i] = false;
    }
    let yes = dp[k];
    println!("{}", if yes { "First" } else { "Second" });
}
