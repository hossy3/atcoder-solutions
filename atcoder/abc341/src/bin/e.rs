use ac_library::{Min, Segtree};
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        (n, q): (usize, usize),
        s: Chars,
        tlr: [(u8, Usize1, Usize1); q],
    }

    // 次の文字に対して良い文字列の条件を満たす (= 異なる) ことを示す
    let mut segtree: Segtree::<Min<_>> = vec![0u8; n].into();
    for (i, v) in s.windows(2).enumerate() {
        if v[0] != v[1] {
            segtree.set(i, 1);
        }
    }

    for &(t, l, r) in &tlr {
        match t {
            1 => {
                if l > 0 {
                    segtree.set(l - 1, 1 - segtree.get(l - 1));
                }
                segtree.set(r, 1 - segtree.get(r));
            }
            2 => {
                let yes = l == r || segtree.prod(l..r) == 1; // all true
                println!("{}", if yes { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
    }
}
