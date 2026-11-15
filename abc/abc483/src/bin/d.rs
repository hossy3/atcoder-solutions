use ac_library::{LazySegtree, MapMonoid, Max, Min};
use proconio::input;

struct MaxAdd;
impl MapMonoid for MaxAdd {
    type M = Max<isize>;
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

fn f(min: isize, max: isize) -> isize {
    let min = min.min(0);
    let max = max.max(0);
    max - min + 1
}

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut sum = vec![0isize; n + 1]; // 累積和
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }

    let mut segtree_min: LazySegtree<MinAdd> = sum.clone().into();
    let mut segtree_max: LazySegtree<MaxAdd> = sum.clone().into();

    // eprintln!("{:?}", &sum);
    // eprintln!("{}", segtree_max.all_prod());
    // eprintln!("{}", segtree_min.all_prod());
    // for j in 0..=n {
    //     eprint!("{} {}, ", segtree_min.get(j), segtree_max.get(j));
    // }
    // eprintln!();

    let mut result = f(segtree_min.all_prod(), segtree_max.all_prod());
    for i in 0..n {
        // i 番目で反転する
        segtree_min.apply_range((i + 1).., -a[i] * 2);
        segtree_max.apply_range((i + 1).., -a[i] * 2);
        // eprintln!("{result}");
        // for j in 0..=n {
        //     eprint!("{} {}, ", segtree_min.get(j), segtree_max.get(j));
        // }
        // eprintln!();
        result = result.max(f(segtree_min.all_prod(), segtree_max.all_prod()));
        segtree_min.apply_range((i + 1).., a[i] * 2);
        segtree_max.apply_range((i + 1).., a[i] * 2);
    }

    println!("{result}");
}
