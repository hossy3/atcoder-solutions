use proconio::input;

/// i 番目の桁が j のときの状態を state[i][j][upper] に記録する
fn digit_dp(a: usize, i: usize, j: usize, upper: usize, state: &mut [[[usize; 2]; 10]]) {
    if state[i][j][upper] != usize::MAX {
        return;
    }
    if j == 4 || j == 9 {
        if upper == 1 {
            state[i][j][upper] = a % 10usize.pow(i as u32) + 1; // これ以下にする
        } else {
            state[i][j][upper] = 10usize.pow(i as u32); // 下の桁はなんでも良い
        }
        return;
    }
    if i == 0 {
        state[i][j][upper] = 0;
        return;
    }

    let i0 = i - 1;
    let mut count = 0;
    if upper == 1 {
        let k0 = a / 10usize.pow(i0 as u32) % 10; // a の i 桁目
        // eprintln!("{a} {i} {j} {upper} {k0}");
        for k in 0..k0 {
            let upper = 0;
            digit_dp(a, i0, k, upper, state);
            count += state[i0][k][upper];
        }
        digit_dp(a, i0, k0, upper, state);
        count += state[i0][k0][upper];
    } else {
        for k in 0..=9 {
            digit_dp(a, i0, k, upper, state);
            count += state[i0][k][upper];
        }
    }

    state[i][j][upper] = count;
}

fn f(a: usize) -> usize {
    if a == 0 {
        return 0;
    }
    let k = a.ilog10() as usize;
    let mut state = vec![[[usize::MAX; 2]; 10]; k + 2];
    digit_dp(a, k + 1, 0, 1, &mut state);
    // eprintln!("{:?}", &state);
    state[k + 1][0][1]
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let result = f(b) - f(a - 1);
    println!("{result}");
}
