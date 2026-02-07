use ac_library::FenwickTree;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

const N: usize = 26;

fn to_index(c: char) -> usize {
    c as usize - 'a' as usize
}

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
    }

    let mut trees = vec![FenwickTree::new(n, 0isize); N];
    for (i, &c) in s.iter().enumerate() {
        trees[to_index(c)].add(i, 1);
    }

    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    i: Usize1,
                    c: char,
                }
                trees[to_index(s[i])].add(i, -1);
                trees[to_index(c)].add(i, 1);
                s[i] = c;
            }
            2 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                }
                let result = trees.iter().filter(|tree| tree.sum(l..=r) > 0).count();
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
