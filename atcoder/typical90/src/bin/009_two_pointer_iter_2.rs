use proconio::input;

trait TwoPointerOps {
    type Item: Clone;
    type State: Clone;
    type IterItem;

    fn init() -> Self::State;
    fn op(a: &[Self::Item], r: usize, state: &Self::State) -> Self::State;
    fn inv_op(a: &[Self::Item], l: usize, state: &Self::State) -> Self::State;
    fn pass(state: &Self::State) -> bool;
    fn conv(state: &Self::State, pass: bool) -> Self::IterItem;
}

struct TwoPointer<Ops>
where
    Ops: TwoPointerOps,
{
    l: usize,
    r: usize,
    pass: bool,
    state: Ops::State,
    v: Vec<Ops::Item>,
}

impl<Ops: TwoPointerOps> Iterator for TwoPointer<Ops> {
    type Item = Ops::IterItem;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.v.len();
        if self.l < n {
            if !self.pass || self.r == n {
                self.state = Ops::inv_op(&self.v, self.l, &self.state);
                self.l += 1;
            } else {
                self.state = Ops::op(&self.v, self.r, &self.state);
                self.r += 1;
            }
        }

        if self.l < n {
            self.pass = Ops::pass(&self.state);
            Some(Ops::conv(&self.state, self.pass))
        } else {
            None
        }
    }
}

impl<Ops: TwoPointerOps> From<Vec<Ops::Item>> for TwoPointer<Ops> {
    fn from(v: Vec<Ops::Item>) -> Self {
        TwoPointer::<Ops> {
            l: 0,
            r: 0,
            pass: true,
            state: Ops::init(),
            v,
        }
    }
}

struct Ops009;
impl TwoPointerOps for Ops009 {
    type Item = f64;
    type State = (f64, f64);
    type IterItem = f64;

    fn init() -> Self::State {
        (0.0, 0.0)
    }
    fn op(a: &[Self::Item], r: usize, &(degl, _): &Self::State) -> Self::State {
        (degl, a[r])
    }
    fn inv_op(a: &[Self::Item], l: usize, &(_, degr): &Self::State) -> Self::State {
        (a[l], degr)
    }
    fn pass(&(degl, degr): &Self::State) -> bool {
        degr - degl <= 180.0
    }
    fn conv(&(degl, degr): &Self::State, _: bool) -> Self::IterItem {
        let deg = degr - degl;
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
    let deg = degs[0];

    let mut tp = TwoPointer::<Ops009>::from(degs);
    tp.state = (deg, deg);
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
