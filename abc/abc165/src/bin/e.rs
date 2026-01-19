use proconio::input;

fn main() {
    input! {
        _: usize,
        m: usize,
    }
    // ABCDDCBA-- のように並び替えたいが、 A--A は C--C と同じ間隔になっている
    // ABBA と CD-DC をつなげた ABBACD-DC- のような配列を作る
    let k1 = m / 2; // 右の個数
    let k0 = m - k1; // 左の個数
    for i in 0..k0 {
        println!("{} {}", i + 1, k0 * 2 - i);
    }
    for i in 0..k1 {
        println!("{} {}", k0 * 2 + i + 1, (k0 + k1) * 2 - i + 1);
    }
}
