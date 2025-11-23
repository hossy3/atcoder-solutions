use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut v0: Vec<usize> = (0..n).collect(); // 鳩のいる巣
    let mut v1 = vec![1usize; n]; // 巣ごとの鳩の数
    let mut count = 0usize;

    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    p: Usize1,
                    h: Usize1,
                }
                let h_prev = v0[p]; // 前に鳩がいた巣
                v1[h_prev] -= 1;
                if v1[h_prev] == 1 {
                    count -= 1;
                }
                v0[p] = h;
                v1[h] += 1;
                if v1[h] == 2 {
                    count += 1;
                }
            }
            2 => {
                println!("{count}");
            }
            _ => unreachable!(),
        }
    }
}
