use proconio::input;

type Pos = (i64, i64, i64);
type Rect = Option<(Pos, Pos)>;

fn cross_line((l0, r0): (i64, i64), (l1, r1): (i64, i64)) -> Option<(i64, i64)> {
    if l0 > l1 {
        return cross_line((l1, r1), (l0, r0));
    }

    if l1 < r0 {
        if r0 <= r1 {
            Some((l1, r0))
        } else {
            Some((l1, r1))
        }
    } else {
        None
    }
}

fn cross_box(r0: Rect, r1: Rect) -> Rect {
    let Some((p0min, p0max)) = r0 else { return None };
    let Some((p1min, p1max)) = r1 else { return None };
    let Some((xmin, xmax)) = cross_line((p0min.0, p0max.0), (p1min.0, p1max.0)) else { return None };
    let Some((ymin, ymax)) = cross_line((p0min.1, p0max.1), (p1min.1, p1max.1)) else { return None };
    let Some((zmin, zmax)) = cross_line((p0min.2, p0max.2), (p1min.2, p1max.2)) else { return None };
    Some(((xmin, ymin, zmin), (xmax, ymax, zmax)))
}

fn volume(r: Rect) -> usize {
    if let Some(((xmin, ymin, zmin), (xmax, ymax, zmax))) = r {
        ((xmax - xmin) * (ymax - ymin) * (zmax - zmin)) as usize
    } else {
        0
    }
}

fn f(p1: Pos, p2: Pos, v: [usize; 3]) -> bool {
    let r0 = Some(((0, 0, 0), (7, 7, 7)));
    let r1 = Some((p1, (p1.0 + 7, p1.1 + 7, p1.2 + 7)));
    let r2 = Some((p2, (p2.0 + 7, p2.1 + 7, p2.2 + 7)));

    let r01 = cross_box(r0, r1);
    let r12 = cross_box(r1, r2);
    let r20 = cross_box(r2, r0);
    let r123 = cross_box(r01, r2);

    let c3 = volume(r123);
    let c2 = volume(r01) + volume(r12) + volume(r20) - c3 * 3;
    let c1 = 7 * 7 * 7 * 3 - c2 * 2 - c3 * 3;
    [c1, c2, c3] == v
}

fn main() {
    input! {
        v: [usize; 3],
    }
    let v = [v[0], v[1], v[2]];
    for x0 in 0..=7 {
        for y0 in 0..=7 {
            for z0 in 0..=7 {
                for x1 in (-6)..=(x0 + 7) {
                    for y1 in (-6)..=(y0 + 7) {
                        for z1 in (-6)..=(z0 + 7) {
                            if f((x0, y0, z0), (x1, y1, z1), v) {
                                println!("Yes");
                                println!("0 0 0 {x0} {y0} {z0} {x1} {y1} {z1}");
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("No");
}
