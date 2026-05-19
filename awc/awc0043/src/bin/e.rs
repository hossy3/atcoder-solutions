use std::collections::HashMap;

use proconio::input;

type Mint = ac_library::ModInt1000000007;

/// 桁 DP をメモ化再帰で解く
/// @see: https://atcoder.jp/contests/awc0043/editorial/18492
///
/// coef: 見ている桁の重み 10^k
/// upper: 現在の桁から下の部分で取りうる値の上限 (半開区間)
/// is_zero: ここまでの桁がすべて 0 か
/// prev: 直前の桁
/// has_zero: 途中の桁に 0 が含まれているか
/// memo: メモ化した値
///
/// 返り値: (条件を満たす整数の個数, スコアの総和)
fn digit_dp(
    coef: usize,
    upper: usize,
    is_zero: bool,
    prev: usize,
    has_zero: bool,
    memo: &mut HashMap<(usize, usize, bool, usize, bool), (Mint, Mint)>,
) -> (Mint, Mint) {
    if upper == 0 {
        return (Mint::new(0), Mint::new(0)); // 区間が空の場合は個数なし
    }

    if coef == 0 {
        if is_zero {
            return (Mint::new(0), Mint::new(0)); // 0 のみの場合は個数なし
        }
        if has_zero {
            return (Mint::new(2), Mint::new(0)); // 途中の桁に 0 が含まれている場合は 2個分と数える
        } else {
            return (Mint::new(1), Mint::new(0)); // 途中の桁に 0 が含まれていない場合は 1個分
        }
    }

    if let Some(&ret) = memo.get(&(coef, upper, is_zero, prev, has_zero)) {
        return ret;
    }

    let mut count = Mint::new(0);
    let mut score = Mint::new(0);
    for i in 0..10 {
        let upper = upper.saturating_sub(i * coef).min(coef);
        let coef = coef / 10;
        if is_zero {
            let is_zero = is_zero && i == 0;
            let (c, s) = digit_dp(coef, upper, is_zero, i, has_zero, memo);
            count += c;
            score += s;
        } else {
            let has_zero = has_zero || i == 0;
            let (c, s) = digit_dp(coef, upper, is_zero, i, has_zero, memo);
            count += c;
            score += s + c * prev.abs_diff(i);
        }
    }

    let ret = (count, score);
    memo.insert((coef, upper, is_zero, prev, has_zero), ret);
    ret
}

fn main() {
    input! {
        n: usize,
    }
    let mut memo = HashMap::new();
    let (_, result) = digit_dp(10usize.pow(18), n + 1, true, 0, false, &mut memo);
    println!("{result}");
}
