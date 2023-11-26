use proconio::{input, marker::Chars};

// m:   0,      1,      2
// e 0: 0,      3(0-1), 4(0-2)
//   1: 3(0-1), 1,      5(1-2)
//   2: 4(0-2), 5(1-2), 2
fn update_es(mes: &mut [usize; 6], ms: &[usize; 3], e: usize) {
    match e {
        0 => {
            mes[0] += ms[0];
            mes[3] += ms[1];
            mes[4] += ms[2];
        }
        1 => {
            mes[3] += ms[0];
            mes[1] += ms[1];
            mes[5] += ms[2];
        }
        _ => {
            mes[4] += ms[0];
            mes[5] += ms[1];
            mes[2] += ms[2];
        }
    }
}

// me:  0, 1, 2, 3(0-1), 4(0-2), 5(1-2)
// x 0: 1, 2, 1, 2,      1,      3
//   1: 2, 0, 0, 2,      3,      0
//   2: 1, 0, 0, 3,      1,      0
fn mex(mes: &[usize; 6], x: usize) -> usize {
    match x {
        0 => mes[0] * 1 + mes[1] * 2 + mes[2] * 1 + mes[3] * 2 + mes[4] * 1 + mes[5] * 3,
        1 => mes[0] * 2 + mes[1] * 0 + mes[2] * 0 + mes[3] * 2 + mes[4] * 3 + mes[5] * 0,
        _ => mes[0] * 1 + mes[1] * 0 + mes[2] * 0 + mes[3] * 3 + mes[4] * 1 + mes[5] * 0,
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars,
    }

    let mut ms = vec![[0, 0, 0]; n + 1]; // 0, 1, 2
    for i in 0..n {
        ms[i + 1] = ms[i];
        if s[i] != 'M' {
            continue;
        }
        ms[i + 1][a[i]] += 1;
    }

    let mut mes = vec![[0, 0, 0, 0, 0, 0]; n + 1]; // 0, 1, 2, 0-1, 0-2, 1-2
    for i in 0..n {
        mes[i + 1] = mes[i];
        if s[i] != 'E' {
            continue;
        }
        update_es(&mut mes[i + 1], &ms[i], a[i]);
    }

    let mut result = 0usize;
    for i in 0..n {
        if s[i] != 'X' {
            continue;
        }
        result += mex(&mes[i], a[i]);
    }

    println!("{}", result);
}
