use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut set0 = BTreeSet::new();
    let mut set1 = BTreeSet::new();
    let mut result = 0usize;

    for i in 0..q {
        input! {
            op: char,
            x: usize,
        }

        match op {
            '+' => {
                if set0.len() == set1.len() {
                    set0.insert((x, i));
                } else {
                    set1.insert((x, i));
                }

                if let Some(&xi0) = set0.last() {
                    if let Some(&xi1) = set1.first() {
                        if xi0 > xi1 {
                            set0.pop_last();
                            set1.pop_first();
                            set0.insert(xi1);
                            set1.insert(xi0);
                        }
                    }
                }
            }
            '-' => {
                if let Some(&(x0, _)) = set0.last() {
                    if x == x0 {
                        result += 1;
                    }
                }
                if let Some(&xi) = set0.range((x, 0)..(x + 1, 0)).next() {
                    set0.remove(&xi);
                    if set0.len() < set1.len() {
                        let Some(xi) = set1.pop_first() else {
                            unreachable!()
                        };
                        set0.insert(xi);
                    }
                } else if let Some(&xi) = set1.range((x, 0)..(x + 1, 0)).next() {
                    set1.remove(&xi);
                    if set0.len() > set1.len() + 1 {
                        let Some(xi) = set0.pop_last() else {
                            unreachable!()
                        };
                        set1.insert(xi);
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{result}");
}
