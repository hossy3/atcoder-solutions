use proconio::input;

fn aoki(a: &[i64], mut v: [u8; 9], tscore: i64, ascore: i64) -> bool {
    if v.iter().all(|&x| x != 0) {
        return ascore > tscore;
    }

    for i in 0..9 {
        if v[i] != 0 {
            continue;
        }
        let j = (i / 3) * 3;
        if (i % 3 == 0 && v[j + 1] == 2 && v[j + 2] == 2)
            || (i % 3 == 1 && v[j] == 2 && v[j + 2] == 2)
            || (i % 3 == 2 && v[j] == 2 && v[j + 1] == 2)
        {
            return true;
        }
        let j = i % 3;
        if (i / 3 == 0 && v[3 + j] == 2 && v[6 + j] == 2)
            || (i / 3 == 1 && v[j] == 2 && v[6 + j] == 2)
            || (i / 3 == 2 && v[j] == 2 && v[3 + j] == 2)
        {
            return true;
        }
        if (i == 0 && v[4] == 2 && v[8] == 2)
            || (i == 2 && v[4] == 2 && v[6] == 2)
            || (i == 4 && v[0] == 2 && v[8] == 2)
            || (i == 4 && v[2] == 2 && v[6] == 2)
            || (i == 6 && v[2] == 2 && v[4] == 2)
            || (i == 8 && v[0] == 2 && v[4] == 2)
        {
            return true;
        }
        v[i] = 2;
        if !takahashi(a, v, tscore, ascore + a[i]) {
            return true;
        }
        v[i] = 0;
    }

    false
}

// v: 0: none, 1: takahashi, 2: aoki
// return: takahashi の勝ち
fn takahashi(a: &[i64], mut v: [u8; 9], tscore: i64, ascore: i64) -> bool {
    for i in 0..9 {
        if v[i] != 0 {
            continue;
        }
        let j = (i / 3) * 3;
        if (i % 3 == 0 && v[j + 1] == 1 && v[j + 2] == 1)
            || (i % 3 == 1 && v[j] == 1 && v[j + 2] == 1)
            || (i % 3 == 2 && v[j] == 1 && v[j + 1] == 1)
        {
            return true;
        }
        let j = i % 3;
        if (i / 3 == 0 && v[3 + j] == 1 && v[6 + j] == 1)
            || (i / 3 == 1 && v[j] == 1 && v[6 + j] == 1)
            || (i / 3 == 2 && v[j] == 1 && v[3 + j] == 1)
        {
            return true;
        }
        if (i == 0 && v[4] == 1 && v[8] == 1)
            || (i == 2 && v[4] == 1 && v[6] == 1)
            || (i == 4 && v[0] == 1 && v[8] == 1)
            || (i == 4 && v[2] == 1 && v[6] == 1)
            || (i == 6 && v[2] == 1 && v[4] == 1)
            || (i == 8 && v[0] == 1 && v[4] == 1)
        {
            return true;
        }
        v[i] = 1;
        if !aoki(a, v, tscore + a[i], ascore) {
            return true;
        }
        v[i] = 0;
    }

    false
}

fn main() {
    input! {
        a: [i64; 9]
    }
    let yes = takahashi(&a, [0; 9], 0, 0);
    println!("{}", if yes { "Takahashi" } else { "Aoki" });
}
