use proconio::input;

fn f(x0: usize, x1: usize, x2: usize, x3: usize) -> bool {
    (x0 < x2 && x2 < x1) || (x2 <= x0 && x0 < x3)
}

#[test]
fn test_func() {
    assert_eq!(f(1, 2, 3, 4), false);
    assert_eq!(f(1, 2, 2, 3), false);
    assert_eq!(f(1, 3, 2, 3), true);
    assert_eq!(f(1, 4, 2, 3), true);

    assert_eq!(f(3, 4, 1, 2), false);
    assert_eq!(f(2, 3, 1, 2), false);
    assert_eq!(f(2, 3, 1, 3), true);
    assert_eq!(f(2, 3, 1, 4), true);
}

fn main() {
    input! {
        n: usize,
        b: [(usize, usize, usize, usize, usize, usize); n],
    }
    let mut xs = vec![vec![]; 101];
    let mut ys = vec![vec![]; 101];
    let mut zs = vec![vec![]; 101];
    for (i, &(x0, y0, z0, x1, y1, z1)) in b.iter().enumerate() {
        xs[x0].push(i);
        ys[y0].push(i);
        zs[z0].push(i);
        xs[x1].push(i);
        ys[y1].push(i);
        zs[z1].push(i);
    }

    for (i, &(x0, y0, z0, x1, y1, z1)) in b.iter().enumerate() {
        let mut result = 0usize;

        for k in 0..2 {
            let xs = if k == 0 { &xs[x0] } else { &xs[x1] };
            for &j in xs {
                if i == j {
                    continue;
                }
                let (_, y2, z2, _, y3, z3) = b[j];
                if f(y0, y1, y2, y3) && f(z0, z1, z2, z3) {
                    result += 1;
                }
            }
        }

        for k in 0..2 {
            let ys = if k == 0 { &ys[y0] } else { &ys[y1] };
            for &j in ys {
                if i == j {
                    continue;
                }
                let (x2, _, z2, x3, _, z3) = b[j];
                if f(x0, x1, x2, x3) && f(z0, z1, z2, z3) {
                    result += 1;
                }
            }
        }

        for k in 0..2 {
            let zs = if k == 0 { &zs[z0] } else { &zs[z1] };
            for &j in zs {
                if i == j {
                    continue;
                }
                let (x2, y2, _, x3, y3, _) = b[j];
                if f(x0, x1, x2, x3) && f(y0, y1, y2, y3) {
                    result += 1;
                }
            }
        }

        println!("{}", result);
    }
}
