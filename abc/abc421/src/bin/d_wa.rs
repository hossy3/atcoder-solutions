use std::collections::VecDeque;

use proconio::input;

fn dir(c: char) -> (i64, i64) {
    match c {
        'U' => (-1, 0),
        'D' => (1, 0),
        'L' => (0, -1),
        'R' => (0, 1),
        _ => unreachable!(),
    }
}

fn main() {
    input! {
        rt: i64,
        ct: i64,
        ra: i64,
        ca: i64,
        _n: usize,
        m: usize,
        l: usize,
        sa: [(char, i64); m],
        tb: [(char, i64); l],
    }

    // st を同じ区切りで分割する
    let mut stk = vec![];
    let mut sa = VecDeque::from(sa);
    let mut tb = VecDeque::from(tb);
    while sa.len() > 0 && tb.len() > 0 {
        let Some((s, a)) = sa.pop_front() else {
            unreachable!()
        };
        let Some((t, b)) = tb.pop_front() else {
            unreachable!()
        };
        if a < b {
            stk.push((s, t, a));
            tb.push_front((t, b - a));
        } else if a > b {
            stk.push((s, t, b));
            sa.push_front((s, a - b));
        } else {
            stk.push((s, t, a));
        }
    }

    let mut result = 0;
    let (mut rt, mut ct) = (rt, ct);
    let (mut ra, mut ca) = (ra, ca);
    for (s, t, k) in stk {
        let dt = dir(s);
        let da = dir(t);
        if s == t {
            if rt == ra && ct == ca {
                result += k;
            }
        } else if s == 'U' && t == 'D' {
            if ct == ca && rt > ra && (rt - ra) % 2 == 0 && (rt - ra) / 2 <= k {
                result += 1;
            }
        } else if s == 'D' && t == 'U' {
            if ct == ca && rt < ra && (ra - rt) % 2 == 0 && (ra - rt) / 2 <= k {
                result += 1;
            }
        } else if s == 'L' && t == 'R' {
            if rt == ra && ct > ca && (ct - ca) % 2 == 0 && (ct - ca) / 2 <= k {
                result += 1;
            }
        } else if s == 'R' && t == 'L' {
            if rt == ra && ct < ca && (ca - ct) % 2 == 0 && (ca - ct) / 2 <= k {
                result += 1;
            }
        } else {
            let k0 = rt.abs_diff(ra) as i64;
            let k1 = ct.abs_diff(ca) as i64;
            if k0 == k1 && k0 <= k {
                let (rt0, ct0) = (rt + dt.0 * k0, ct + dt.1 * k0);
                let (ra0, ca0) = (ra + da.0 * k1, ca + da.1 * k1);
                if rt0 == ra0 && ct0 == ca0 {
                    result += 1;
                }
            }
        }

        rt += dt.0 * k;
        ct += dt.1 * k;
        ra += da.0 * k;
        ca += da.1 * k;
    }

    println!("{result}");
    eprintln!("{rt} {ct}, {ra} {ca}");
}
