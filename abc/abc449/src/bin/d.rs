use proconio::input;

fn main() {
    input! {
        l: isize,
        r: isize,
        d: isize,
        u: isize,
    }

    let include = |x: isize, y: isize| -> bool { l <= x && x <= r && d <= y && y <= u };
    let dist = |xy0: (isize, isize), xy1: (isize, isize)| -> isize {
        (xy1.0 - xy0.0).abs() + (xy1.1 - xy0.1).abs()
    };

    let mut result = 0isize;
    for i in (0..=1_000_000_isize).step_by(2) {
        if i == 0 {
            if include(0, 0) {
                result += 1;
            }
            continue;
        }

        // 4頂点とその辺について黒の数を足す
        for k in 0..4 {
            // eprintln!("{result}");
            let ((x0, y0), (x1, y1), (x2, y2), (x3, y3)) = match k {
                0 => ((i, i), (-i, i), (i.min(r), i), ((-i).max(l), i)),
                1 => ((-i, i), (-i, -i), (-i, i.min(u)), (-i, (-i).max(d))),
                2 => ((-i, -i), (i, -i), ((-i).max(l), -i), (i.min(r), -i)),
                3 => ((i, -i), (i, i), (i, (-i).max(d)), (i, i.min(u))),
                _ => unreachable!(),
            };
            if include(x0, y0) {
                if include(x1, y1) {
                    result += i * 2;
                } else {
                    let x = dist((x0, y0), (x3, y3));
                    if x < i * 2 {
                        result += x + 1;
                    }
                }
            } else {
                if include(x1, y1) {
                    let x = dist((x1, y1), (x2, y2));
                    if x < i * 2 {
                        result += x;
                    }
                } else {
                    result += match k {
                        0 => {
                            if -i < l && r < i && d <= i && i <= u {
                                r - l + 1
                            } else {
                                0
                            }
                        }
                        1 => {
                            if l <= -i && -i <= r && -i < d && u < i {
                                u - d + 1
                            } else {
                                0
                            }
                        }
                        2 => {
                            if -i < l && r < i && d <= -i && -i <= u {
                                r - l + 1
                            } else {
                                0
                            }
                        }
                        3 => {
                            if l <= i && i <= r && -i < d && u < i {
                                u - d + 1
                            } else {
                                0
                            }
                        }
                        _ => unreachable!(),
                    };
                }
            }
        }
    }

    println!("{result}");
}
