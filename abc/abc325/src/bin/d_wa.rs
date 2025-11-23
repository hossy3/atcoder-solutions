use btreemultimap::BTreeMultiMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        td: [(usize, usize); n],
    }
    let mut map = BTreeMultiMap::new();
    for &(t, d) in &td {
        map.insert((t, d), {});
    }
    let mut result = 0;
    let mut cur = 1usize;
    for (&(t, d), _) in &map {
        if cur > t + d {
            continue;
        }
        result += 1;
        cur = cur.max(t) + 1;
    }
    println!("{result}");
}
