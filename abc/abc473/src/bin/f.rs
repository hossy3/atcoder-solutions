use ac_library::{LazySegtree, MapMonoid, Min};
use proconio::{
    input,
    marker::{Chars, Usize1},
};

struct MinAdd;
impl MapMonoid for MinAdd {
    type M = Min<isize>;
    type F = isize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &isize, &x: &isize) -> isize {
        f + x
    }

    fn composition(&f: &isize, &g: &isize) -> isize {
        f + g
    }
}

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
    }

    let mut sum = vec![0isize; n + 1];
    for (i, &c) in s.iter().enumerate() {
        sum[i + 1] = sum[i] + if c == 'A' { 1 } else { -1 };
    }
    let mut segtree: LazySegtree<MinAdd> = sum.into();

    for _ in 0..q {
        input! {
            k: u8,
        }

        match k {
            1 => {
                input! {
                    i: Usize1,
                    c: char,
                }
                if s[i] == 'A' && c == 'B' {
                    segtree.apply_range((i + 1).., -2);
                } else if s[i] == 'B' && c == 'A' {
                    segtree.apply_range((i + 1).., 2);
                }
                s[i] = c;
            }
            2 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                }
                let min = segtree.prod((l + 1)..=(r + 1));
                let x = segtree.get(l);
                let yes = min >= x;
                println!("{}", if yes { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
    }
}
