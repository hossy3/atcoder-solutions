use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        vac: [(u8, usize, usize); n],
    }

    let mut results1 = vec![usize::MAX; x + 1];
    let mut results2 = vec![usize::MAX; x + 1];
    let mut results3 = vec![usize::MAX; x + 1];

    results1[0] = 0;
    results2[0] = 0;
    results3[0] = 0;

    for &(v, a, c) in &vac {
        for i in (0..=(x - c)).rev() {
            let j = i + c;
            match v {
                1 => {
                    if results1[i] == usize::MAX {
                        continue;
                    }
                    let x = results1[i] + a;
                    if results1[j] == usize::MAX || results1[j] < x {
                        results1[j] = x;
                    }
                }
                2 => {
                    if results2[i] == usize::MAX {
                        continue;
                    }
                    let x = results2[i] + a;
                    if results2[j] == usize::MAX || results2[j] < x {
                        results2[j] = x;
                    }
                }
                3 => {
                    if results3[i] == usize::MAX {
                        continue;
                    }
                    let x = results3[i] + a;
                    if results3[j] == usize::MAX || results3[j] < x {
                        results3[j] = x;
                    }
                }
                _ => unreachable!(),
            };
        }
    }

    let mut x3s = vec![0usize; x + 1];
    for i in 1..=x {
        if results3[i] == usize::MAX {
            x3s[i] = x3s[i - 1];
        } else {
            x3s[i] = x3s[i - 1].max(results3[i]);
        }
    }

    let mut result = 0usize;
    for i1 in 0..=x {
        if results1[i1] == usize::MAX {
            continue;
        }
        for i2 in 0..=(x - i1) {
            if results2[i2] == usize::MAX {
                continue;
            }
            let i3 = x - i1 - i2;
            result = result.max(results1[i1].min(results2[i2]).min(x3s[i3]));
        }
    }
    println!("{result}");
}
