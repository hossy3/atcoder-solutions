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

struct Ops076;
impl TwoPointerOps for Ops076 {
    type Item = usize;
    type State = (usize, usize);
    type IterItem = bool;

    fn init() -> Self::State {
        (0, 0)
    }
    fn op(a: &[Self::Item], r: usize, &(s, k): &Self::State) -> Self::State {
        (s + a[r], k)
    }
    fn inv_op(a: &[Self::Item], l: usize, &(s, k): &Self::State) -> Self::State {
        (s - a[l], k)
    }
    fn pass(&(s, k): &Self::State) -> bool {
        s <= k
    }
    fn conv(&(s, k): &Self::State, _: bool) -> Self::IterItem {
        s == k
    }
}

fn f(a: &[usize]) -> bool {
    let sum: usize = a.iter().sum();
    if sum % 10 != 0 {
        return false;
    }
    let target = sum / 10;

    let v: Vec<_> = a.iter().chain(a.iter()).map(|&x| x).collect();
    let mut tp = TwoPointer::<Ops076>::from(v);
    tp.state.1 = target;

    tp.any(|b| b)
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let yes = f(&a);
    println!("{}", if yes { "Yes" } else { "No" });
}
