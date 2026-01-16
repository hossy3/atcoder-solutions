use itertools::Itertools;
use proconio::{input, marker::Chars};

// インデントの (最小値, 最終値) を返す
fn g(s: &[char]) -> (isize, isize) {
    let mut min = 0isize;
    let mut cum = 0isize;
    for &c in s {
        match c {
            '(' => {
                cum += 1;
            }
            ')' => {
                cum -= 1;
                min = min.min(cum)
            }
            _ => unreachable!(),
        }
    }

    (min, cum)
}

fn f(s: &[Vec<char>]) -> bool {
    let mut v_up = vec![]; // 上り方向 (((
    let mut v_down = vec![]; // 下り方向 )))

    for s in s {
        let (min, cum) = g(s);
        if cum >= 0 {
            v_up.push((min, cum));
        } else {
            v_down.push((min - cum, -cum)); // 逆方向から上るという扱いにする
        }
    }

    if v_up.iter().map(|&(_, cum)| cum).sum::<isize>()
        != v_down.iter().map(|&(_, cum)| cum).sum::<isize>()
    {
        return false; // 全部つなげて 0 に戻らないのはダメ
    }

    // min 条件がだんだんゆるくなるので min が大きな順に処理すれば良い
    for v in [v_up, v_down] {
        let mut cur = 0isize;
        for (min, cum) in v.iter().sorted().rev() {
            if cur + min < 0 {
                return false; // 0 より下がってしまうのはダメ
            }
            cur += cum;
        }
    }

    true
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let yes = f(&s);
    println!("{}", if yes { "Yes" } else { "No" });
}
