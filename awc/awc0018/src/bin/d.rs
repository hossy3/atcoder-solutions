use fixedbitset::FixedBitSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: [usize; n],
    }

    let mut state = FixedBitSet::with_capacity(k + 1);
    state.insert(0);
    for x in c {
        if x > k {
            continue; // 1つ購入だけで予算を超えてしまう
        }
        for i in (0..=(k - x)).rev() {
            if state.contains(i) {
                state.insert(i + x);
            }
        }
    }
    // eprintln!("{:?}", &state);

    let result = (0..=k).rfind(|&i| state.contains(i)).unwrap();
    println!("{result}");
}
