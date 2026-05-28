use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut ab: [(isize, isize); n],
    }

    'outer: for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    i: Usize1,
                    a: isize,
                    b: isize,
                }
                ab[i] = (a, b);
            }
            2 => {
                input! {
                    d: isize,
                }

                let mut max = 0;
                let mut cur = 0;
                for (i, &(a, b)) in ab.iter().enumerate() {
                    cur += b - a; // 高橋君がどれだけ青木君に近づいたか
                    if cur >= d {
                        println!("{}", i + 1);
                        continue 'outer;
                    }
                    max = cur.max(max);
                }
                if cur <= 0 {
                    println!("-1"); // 1周して近づけないなら永遠に追いつけない
                    continue 'outer;
                }

                let loop_count = (d - max + cur - 1) / cur;
                // eprintln!("{}, {}, {}", cur, max, loop_count);
                cur = cur * loop_count;
                for (i, &(a, b)) in ab.iter().enumerate() {
                    cur += b - a;
                    if cur >= d {
                        println!("{}", loop_count as usize * n + i + 1);
                        continue 'outer;
                    }
                }
                unreachable!()
            }
            _ => unreachable!(),
        }
    }
}
