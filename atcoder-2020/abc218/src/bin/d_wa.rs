use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }
    let mut map = HashMap::new();
    for (i, &(x, y)) in xy.iter().enumerate() {
        map.insert((x, y), i);
    }
    let mut result = 0;
    for i0 in 0..(n - 3) {
        let (x0, y0) = xy[i0];
        for i1 in (i0 + 1)..(n - 2) {
            let (x1, y1) = xy[i1];
            if x0 == x1 {
                for i2 in (i1 + 1)..(n - 1) {
                    let (x2, y2) = xy[i2];
                    if y0 == y2 {
                        if let Some(&i3) = map.get(&(x2, y1)) {
                            if i3 > i2 {
                                result += 1;
                            }
                        }
                    } else if y1 == y2 {
                        if let Some(&i3) = map.get(&(x2, y0)) {
                            if i3 > i2 {
                                result += 1;
                            }
                        }
                    }
                }
            } else if y0 == y1 {
                for i2 in (i1 + 1)..(n - 1) {
                    let (x2, y2) = xy[i2];
                    if x0 == x2 {
                        if let Some(&i3) = map.get(&(x1, y2)) {
                            if i3 > i2 {
                                result += 1;
                            }
                        }
                    } else if x1 == x2 {
                        if let Some(&i3) = map.get(&(x0, y2)) {
                            if i3 > i2 {
                                result += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", result);
}
