use std::collections::BTreeMap;

use proconio::input;

pub mod vec2i {
    use std::ops::{Add, Sub};

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Vec2i {
        x: i128,
        y: i128,
    }

    impl Add for Vec2i {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Sub for Vec2i {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl Vec2i {
        pub fn new(x: i128, y: i128) -> Vec2i {
            Vec2i { x, y }
        }

        pub fn inner_product(&self, other: &Self) -> i128 {
            self.x * other.x + self.y * other.y
        }

        pub fn outer_product(&self, other: &Self) -> i128 {
            self.x * other.y - self.y * other.x
        }

        pub fn x(&self) -> i128 {
            self.x
        }

        pub fn y(&self) -> i128 {
            self.y
        }
    }
}

use vec2i::Vec2i;

fn main() {
    input! {
        n: usize,
        xy: [(i128, i128); n],
        q: usize,
        ab: [(i128, i128); q],
    }
    let n0 = n as i128;

    let mut gx = 0;
    let mut gy = 0;
    for i in 0..n {
        gx += xy[i].0;
        gy += xy[i].1;
    }
    let g = Vec2i::new(gx, gy);

    let mut map = BTreeMap::new();
    for i in 0..n {
        let rad = (((xy[i].1 * n0 - gy) as f64).atan2((xy[i].0 * n0 - gx) as f64) * 1e12) as i128;
        map.insert(rad, i);
    }

    for i in 0..q {
        let rad = (((ab[i].1 * n0 - gy) as f64).atan2((ab[i].0 * n0 - gx) as f64) * 1e12) as i128;
        let v0 = Vec2i::new(ab[i].0 * n0, ab[i].1 * n0);
        let (i, j) = if let Some(x) = map.range(..=rad).last() {
            (*x.1, (x.1 + 1) % n)
        } else {
            (*map.iter().last().unwrap().1, *map.iter().next().unwrap().1)
        };
        let v1 = Vec2i::new(xy[i].0 * n0, xy[i].1 * n0);
        let v2 = Vec2i::new(xy[j].0 * n0, xy[j].1 * n0);
        if v0 == v1 || v0 == v2 {
            println!("ON");
            continue;
        }
        if (v1.x() - v2.x()) * (v0.y() - v2.y()) == (v0.x() - v2.x()) * (v1.y() - v2.y()) {
            println!("ON");
            continue;
        }
        let inner = (v1 - g).outer_product(&(v2 - g))
            > (v1 - g).outer_product(&(v0 - g)) + (v0 - g).outer_product(&(v2 - g));
        println!("{}", if inner { "IN" } else { "OUT" });
    }
}
