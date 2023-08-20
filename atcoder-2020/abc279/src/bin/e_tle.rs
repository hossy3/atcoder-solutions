use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        _: usize,
        m: usize,
        a: [usize; m],
    }

    let mut b = vec![0; m + 1];
    b[0] = 1;
    for i in 0..m {
        let x = a[i];
        let mut pos = b[i];
        if pos == x {
            pos += 1;
        } else if pos == x + 1 {
            pos -= 1;
        }
        b[i + 1] = pos;
    }

    let mut h = HashMap::new();

    'outer: for i in 0..m {
        if b[i] == b[i + 1] {
            println!("{}", b[m]);
            continue;
        }

        let mut pos = b[i];
        for j in (i + 1)..m {
            let x = a[j];
            if pos == x {
                pos += 1;
            } else if pos == x + 1 {
                pos -= 1;
            }
            if let Some(&pos0) = h.get(&(j, pos)) {
                println!("{}", pos0);
                continue 'outer;
            }
        }
        println!("{}", pos);

        let mut pos0 = b[i];
        for j in (i + 1)..m {
            let x = a[j];
            if pos0 == x {
                pos0 += 1;
            } else if pos0 == x + 1 {
                pos0 -= 1;
            }
            h.insert((j, pos0), pos);
        }
    }
}
