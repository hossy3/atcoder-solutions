use proconio::input;
use std::collections::HashMap;

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

struct Ops034;
impl TwoPointerOps for Ops034 {
    type VItem = usize;
    type C = usize;
    type S = HashMap<usize, usize>;
    type IItem = usize;

    fn op(v: &[Self::VItem], r: usize, _: &Self::C, s: &mut Self::S) {
        *s.entry(v[r]).or_insert(0) += 1;
    }
    fn inv_op(v: &[Self::VItem], l: usize, _: &Self::C, s: &mut Self::S) {
        let Some(x) = s.get_mut(&v[l]) else { unreachable!() };
        *x -= 1;
        if *x == 0 {
            s.remove(&v[l]);
        }
    }
    fn pass(_: &[Self::VItem], _: (usize, usize), c: &Self::C, s: &Self::S) -> bool {
        s.len() <= *c
    }
    fn conv(_: &[Self::VItem], (l, r): (usize, usize), c: &Self::C, s: &Self::S) -> Self::IItem {
        if s.len() <= *c {
            r - l
        } else {
            0
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let tp = TwoPointer::<Ops034>::from((a, k, HashMap::new()));
    let max_len: usize = tp.max().unwrap();
    println!("{max_len}");
}
