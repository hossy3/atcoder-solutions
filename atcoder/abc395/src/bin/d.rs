use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut v: Vec<_> = (0..n).collect(); // 鳩aが巣'aにいる
    let mut atob: Vec<_> = (0..n).collect(); // 巣'aは巣b
    let mut btoa: Vec<_> = (0..n).collect(); // 巣bは巣'a

    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }
                v[a] = btoa[b];
            }
            2 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }
                let (a0, b0) = (btoa[a], btoa[b]);
                atob.swap(a0, b0);
                btoa.swap(a, b);
            }
            3 => {
                input! {
                    a: Usize1,
                }
                let result = atob[v[a]] + 1;
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
