use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut v = vec![0usize]; // 先頭の位置
    let mut removed = 0; // 除いた個数
    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    l: usize,
                }
                let Some(&prev) = v.last() else {
                    unreachable!()
                };
                v.push(prev + l);
            }
            2 => {
                removed += 1;
            }
            3 => {
                input! {
                    k: usize,
                }
                let result = v[k + removed - 1] - v[removed];
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
