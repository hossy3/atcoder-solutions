use proconio::input;

trait TwoPointerOps {
    fn is_valid(&self) -> bool;
}

struct TwoPointer {
    n: usize,
    l: usize,
    r: usize,
    valid: bool,

    a: Vec<f64>,
}

impl TwoPointerOps for TwoPointer {
    fn is_valid(&self) -> bool {
        self.l < self.n && self.a[self.r - 1] - self.a[self.l] <= 180.0
    }
}

impl From<Vec<f64>> for TwoPointer {
    fn from(a: Vec<f64>) -> Self {
        TwoPointer {
            n: a.len(),
            l: 0,
            r: 0,
            valid: true,

            a,
        }
    }
}

impl Iterator for TwoPointer {
    type Item = (usize, usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.l < self.n {
            if !self.valid || self.r == self.n {
                self.l += 1;
            } else {
                self.r += 1;
            }
            self.valid = self.is_valid();
        }

        if self.l < self.n {
            Some((self.l, self.r, self.valid))
        } else {
            None
        }
    }
}

fn f(i: usize, xy: &[(f64, f64)]) -> f64 {
    let n = xy.len();
    let mut degs = Vec::with_capacity(n - 1);
    for j in 0..n {
        if i == j {
            continue;
        }
        let rad = (xy[j].1 - xy[i].1).atan2(xy[j].0 - xy[i].0);
        let deg = rad.to_degrees();
        degs.push(deg);
    }
    degs.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let score = TwoPointer::from(degs.clone())
        .map(|(l, r, _)| {
            let deg = degs[r - 1] - degs[l];
            deg.min(360.0 - deg)
        })
        .max_by(|x, y| x.partial_cmp(&y).unwrap())
        .unwrap();
    score
}

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    };
    let score = (0..n).fold(0f64, |acc, i| acc.max(f(i, &xy)));
    println!("{score}");
}
