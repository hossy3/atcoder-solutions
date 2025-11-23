use proconio::{input, marker::Chars};

fn main() {
    input! {
        ha: usize,
        wa: usize,
        a: [Chars; ha],
        hb: usize,
        wb: usize,
        b: [Chars; hb],
        hx: usize,
        wx: usize,
        x: [Chars; hx],
    }

    for a0 in (-9)..=9 {
        for a1 in (-9)..9 {
            for b0 in (-9)..9 {
                'outer: for b1 in (-9)..9 {
                    for c0 in 0..hx {
                        for c1 in 0..wx {
                            if x[c0][c1] == '.' {
                                let a0 = a0 + c0 as i64;
                                let a1 = a1 + c1 as i64;
                                if (0 <= a0 && a0 < ha as i64) && (0 <= a1 && a1 < wa as i64) {
                                    if a[a0 as usize][a1 as usize] == '#' {
                                        continue 'outer;
                                    }
                                }
                                let b0 = b0 + c0 as i64;
                                let b1 = b1 + c1 as i64;
                                if (0 <= b0 && b0 < hb as i64) && (0 <= b1 && b1 < wb as i64) {
                                    if b[b0 as usize][b1 as usize] == '#' {
                                        continue 'outer;
                                    }
                                }
                            } else {
                                let a0 = a0 + c0 as i64;
                                let a1 = a1 + c1 as i64;
                                if (0 <= a0 && a0 < ha as i64) && (0 <= a1 && a1 < wa as i64) {
                                    if a[a0 as usize][a1 as usize] == '#' {
                                        continue;
                                    }
                                }
                                let b0 = b0 + c0 as i64;
                                let b1 = b1 + c1 as i64;
                                if (0 <= b0 && b0 < hb as i64) && (0 <= b1 && b1 < wb as i64) {
                                    if b[b0 as usize][b1 as usize] == '#' {
                                        continue;
                                    }
                                }
                                continue 'outer;
                            }
                        }
                    }
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
