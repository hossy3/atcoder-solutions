use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        t: usize,
        mut a: [usize; n],
    }

    a.sort_unstable();
    while let Some(&x) = a.last() {
        if x < t {
            break;
        }
        a.pop(); // 時刻 T 以降の青木君は考えなくて良い
    }
    a.push(t);

    let mut cur = 0usize; // この時刻まではお湯が出続ける
    let mut count = 0usize;

    for x in a {
        if cur < x {
            count += (x - cur).div_ceil(k);
        }
        cur = x + k;
    }

    println!("{count}");
}
