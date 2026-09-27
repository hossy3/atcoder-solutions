use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    }

    let mut b = vec![false; n]; // おかしを食べたか
    let mut cur = 0usize; // 直近 M 日間で食べたおやつのカロリーの合計
    for (i, &x) in a.iter().enumerate() {
        if i >= m && b[i - m] {
            cur -= a[i - m];
        }
        if cur + x <= k {
            cur += x;
            b[i] = true;
            println!("Yes");
        } else {
            println!("No");
        }
        // eprintln!("{cur}");
    }
}
