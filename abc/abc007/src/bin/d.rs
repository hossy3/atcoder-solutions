use proconio::input;

/// i 番目の桁が j のときの状態を state[i][j][upper] に記録する
/// 状態: ((4, 9 ともに含まない数), (4, 9 を含む数))
fn digit_dp(a: usize, i: usize, j: usize, upper: usize, state: &mut [[[(usize, usize); 2]; 10]]) {
    if state[i][j][upper] != (usize::MAX, usize::MAX) {
        return;
    }
    state[i][j][upper] = (0, 0);

    let b = j == 4 || j == 9;
    if i == 0 {
        if b {
            state[i][j][upper].1 = 1;
        } else {
            state[i][j][upper].0 = 1;
        }
        return;
    }

    let i0 = i - 1;
    let mut update = |k: usize, upper0: usize| {
        digit_dp(a, i0, k, upper0, state);
        if b {
            state[i][j][upper].1 += state[i0][k][upper0].0 + state[i0][k][upper0].1;
        } else {
            state[i][j][upper].0 += state[i0][k][upper0].0;
            state[i][j][upper].1 += state[i0][k][upper0].1;
        }
    };

    if upper == 1 {
        let k0 = a / 10usize.pow(i0 as u32) % 10; // a の i 桁目
        for k in 0..k0 {
            let upper = 0;
            update(k, upper);
        }
        update(k0, upper);
    } else {
        for k in 0..=9 {
            update(k, upper);
        }
    }
}

fn f(a: usize) -> usize {
    if a == 0 {
        return 0;
    }
    let k = a.ilog10() as usize;
    let mut state = vec![[[(usize::MAX, usize::MAX); 2]; 10]; k + 2];
    digit_dp(a, k + 1, 0, 1, &mut state);
    state[k + 1][0][1].1
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let result = f(b) - f(a - 1);
    println!("{result}");
}
