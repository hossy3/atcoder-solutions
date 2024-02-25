use proconio::input;
use std::collections::HashMap;

type S = HashMap<usize, usize>;

trait TwoPointerOps {
    fn update_state_on_expand(&mut self);
    fn update_state_on_shrink(&mut self);
    fn is_valid(&self, state: &S) -> bool;
}

struct TwoPointer {
    n: usize,
    l: usize,
    r: usize,
    state: S,
    valid: bool,

    a: Vec<usize>,
    k: usize,
}

impl TwoPointerOps for TwoPointer {
    fn update_state_on_expand(&mut self) {
        *self.state.entry(self.a[self.r]).or_insert(0) += 1;
    }

    fn update_state_on_shrink(&mut self) {
        if let Some(x) = self.state.get_mut(&self.a[self.l]) {
            *x -= 1;
            if *x == 0 {
                self.state.remove(&self.a[self.l]);
            }
        };
    }

    fn is_valid(&self, state: &S) -> bool {
        state.len() <= self.k
    }
}

impl From<(usize, Vec<usize>)> for TwoPointer {
    fn from((k, a): (usize, Vec<usize>)) -> Self {
        TwoPointer {
            n: a.len(),
            l: 0,
            r: 0,
            state: S::new(),
            valid: true,

            k,
            a,
        }
    }
}

impl Iterator for TwoPointer {
    type Item = (usize, usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.l < self.n {
            if !self.valid || self.r == self.n {
                self.update_state_on_shrink();
                self.l += 1;
            } else {
                self.update_state_on_expand();
                self.r += 1;
            }
            self.valid = self.is_valid(&self.state);
        }

        if self.l < self.n {
            Some((self.l, self.r, self.valid))
        } else {
            None
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let max_len = TwoPointer::from((k, a))
        .map(|(l, r, valid)| if valid { r - l } else { 0 })
        .max()
        .unwrap();
    println!("{max_len}");
}
