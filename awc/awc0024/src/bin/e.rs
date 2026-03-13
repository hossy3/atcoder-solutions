use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        lc: [(usize, usize); n],
    }

    let mut state = vec![usize::MAX; w + 1];
    state[0] = 0;

    for &(l, mut c) in &lc {
        // 二進分割してそれぞれ処理する
        let mut k = 0;
        while c > 0 {
            let c0 = c.min(1 << k);
            c -= c0;
            let l0 = l * c0;
            if l0 > w {
                break;
            }
            k += 1;

            for i in (0..=(w - l0)).rev() {
                if state[i] != usize::MAX {
                    state[i + l0] = state[i + l0].min(state[i] + c0);
                }
            }
        }
    }

    if state[w] == usize::MAX {
        println!("-1");
    } else {
        println!("{}", state[w]);
    }
}
