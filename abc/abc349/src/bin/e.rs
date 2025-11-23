use proconio::input;

fn f(v: &[u8; 9], k: u8) -> bool {
    for i in 0..9 {
        if v[i] != 0 {
            continue;
        }

        let j = (i / 3) * 3;
        if (i % 3 == 0 && v[j + 1] == k && v[j + 2] == k)
            || (i % 3 == 1 && v[j] == k && v[j + 2] == k)
            || (i % 3 == 2 && v[j] == k && v[j + 1] == k)
        {
            return true;
        }

        let j = i % 3;
        if (i / 3 == 0 && v[3 + j] == k && v[6 + j] == k)
            || (i / 3 == 1 && v[j] == k && v[6 + j] == k)
            || (i / 3 == 2 && v[j] == k && v[3 + j] == k)
        {
            return true;
        }
    }

    if (v[0] == 0 && v[4] == k && v[8] == k)
        || (v[2] == 0 && v[4] == k && v[6] == k)
        || (v[4] == 0 && v[0] == k && v[8] == k)
        || (v[4] == 0 && v[2] == k && v[6] == k)
        || (v[6] == 0 && v[2] == k && v[4] == k)
        || (v[8] == 0 && v[0] == k && v[4] == k)
    {
        return true;
    }

    false
}

fn aoki(a: &[i64], mut v: [u8; 9], tscore: i64, ascore: i64) -> bool {
    if v.iter().all(|&x| x != 0) {
        return ascore > tscore;
    }
    if f(&v, 2) {
        return true;
    }

    for i in 0..9 {
        if v[i] != 0 {
            continue;
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
    if f(&v, 1) {
        return true;
    }

    for i in 0..9 {
        if v[i] != 0 {
            continue;
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
