use std::io::{stdin, stdout, BufReader, Write};

use itertools::Itertools;
use proconio::{input, source::line::LineSource};

// interactive

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
        k: usize,
    }

    // input 1st step
    let mut v = Vec::with_capacity(n);
    for i in 0..=k {
        let mut v0 = Vec::with_capacity(k);
        for j in 0..=k {
            if i != j {
                v0.push(j + 1);
            }
        }
        println!("? {}", v0.iter().join(" "));
        stdout().flush().unwrap();

        input! {
            from &mut source,
            x: usize,
        }
        v.push(x);
    }

    // input 2nd step
    for i in (k + 1)..n {
        let mut v0 = Vec::with_capacity(k);
        for j in 0..(k - 1) {
            v0.push(j + 1);
        }
        v0.push(i + 1);
        println!("? {}", v0.iter().join(" "));
        stdout().flush().unwrap();

        input! {
            from &mut source,
            x: usize,
        }
        v.push(x);
    }

    // solve 1st step
    let mut r0 = vec![0; n]; // assume r[0] = 0
    let mut r1 = vec![0; n]; // assume r[0] = 1
    r1[0] = 1;
    for i in 0..k {
        if v[i + 1] == v[i] {
            r0[i + 1] = r0[i];
            r1[i + 1] = r1[i];
        } else {
            r0[i + 1] = 1 - r0[i];
            r1[i + 1] = 1 - r1[i];
        }
    }

    let sum0 = r0[0..(k - 1)].iter().sum::<usize>() % 2;
    let sum1 = r1[0..(k - 1)].iter().sum::<usize>() % 2;

    // solve 2nd step
    for i in (k + 1)..n {
        if v[i] == sum0 {
            r0[i] = 0;
        } else {
            r0[i] = 1;
        }
        if v[i] == sum1 {
            r1[i] = 0;
        } else {
            r1[i] = 1;
        }
    }

    // check r0
    {
        for i in 0..=k {
            let mut x = 0;
            for j in 0..=k {
                if i != j {
                    x += r0[j];
                }
            }
            if x % 2 != v[i] {
                println!("! {}", r1.iter().join(" "));
                stdout().flush().unwrap();
                return;
            }
        }

        for i in (k + 1)..n {
            let mut x = 0;
            for j in 0..(k - 1) {
                x += r0[j];
            }
            x += r0[i];
            if x % 2 != v[i] {
                println!("! {}", r1.iter().join(" "));
                stdout().flush().unwrap();
                return;
            }
        }
    }

    println!("! {}", r0.iter().join(" "));
}
