use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut v: [u128; n],
        dt: [(u128, u128); m],
    }

    v.sort_unstable(); // 各スタッフを作業効率順に並べる

    // t/d の順に並び替える。ソート失敗が予想されるので定数倍しておく
    const K: u128 = 1u128 << 64;
    let mut dti = vec![];
    for (i, &(d, t)) in dt.iter().enumerate() {
        dti.push((t * K / d, i));
    }
    dti.sort_unstable();
    dti.reverse();

    let mut result = 0usize;
    while let Some(v) = v.pop() {
        while let Some((_, i)) = dti.pop() {
            let (d, t) = dt[i];
            if v * t >= d {
                result += 1;
                break;
            }
        }
    }

    println!("{result}");
}
