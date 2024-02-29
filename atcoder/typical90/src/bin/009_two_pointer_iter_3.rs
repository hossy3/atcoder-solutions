use proconio::input;

trait TwoPointerOps {
    type VItem;
    type C;
    type S;
    type IItem;

    fn op(v: &[Self::VItem], r: usize, c: &Self::C, s: &mut Self::S);
    fn inv_op(v: &[Self::VItem], l: usize, c: &Self::C, s: &mut Self::S);
    fn pass(v: &[Self::VItem], lr: (usize, usize), c: &Self::C, s: &Self::S) -> bool;
    fn conv(v: &[Self::VItem], lr: (usize, usize), c: &Self::C, s: &Self::S) -> Self::IItem;
}

struct TwoPointer<Ops>
where
    Ops: TwoPointerOps,
{
    v: Vec<Ops::VItem>,
    l: usize,
    r: usize,
    c: Ops::C,
    s: Ops::S,
}

impl<Ops: TwoPointerOps> Iterator for TwoPointer<Ops> {
    type Item = Ops::IItem;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.v.len();
        let (v, l, r, c) = (&self.v, self.l, self.r, &self.c);
        if l < n {
            let op = (r == 0) || ((r < n) && Ops::pass(v, (l, r), c, &self.s));
            if op {
                Ops::op(v, r, c, &mut self.s);
                self.r += 1;
            } else {
                Ops::inv_op(v, l, c, &mut self.s);
                self.l += 1;
            }
        }

        let (l, r) = (self.l, self.r);
        if self.l < n {
            Some(Ops::conv(v, (l, r), c, &self.s))
        } else {
            None
        }
    }
}

impl<Ops: TwoPointerOps> From<(Vec<Ops::VItem>, Ops::C, Ops::S)> for TwoPointer<Ops> {
    fn from((v, c, s): (Vec<Ops::VItem>, Ops::C, Ops::S)) -> Self {
        let (l, r) = (0, 0);
        TwoPointer::<Ops> { v, l, r, c, s }
    }
}

struct Ops009;
impl TwoPointerOps for Ops009 {
    type VItem = f64;
    type C = ();
    type S = ();
    type IItem = f64;

    fn op(_: &[Self::VItem], _: usize, _: &Self::C, _: &mut Self::S) {}
    fn inv_op(_: &[Self::VItem], _: usize, _: &Self::C, _: &mut Self::S) {}
    fn pass(v: &[Self::VItem], (l, r): (usize, usize), _: &Self::C, _: &Self::S) -> bool {
        v[r - 1] - v[l] <= 180.0
    }
    fn conv(v: &[Self::VItem], (l, r): (usize, usize), _: &Self::C, _: &Self::S) -> Self::IItem {
        let deg = v[r - 1] - v[l];
        deg.min(360.0 - deg)
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

    let tp = TwoPointer::<Ops009>::from((degs, (), ()));
    let score = tp.max_by(|x, y| x.partial_cmp(&y).unwrap()).unwrap();
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
