use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    for (i0, (x0, y0)) in xy.iter().enumerate() {
        let mut j = i0;
        let mut l = 0;
        for (i1, (x1, y1)) in xy.iter().enumerate() {
            if i0 == i1 {
                continue;
            }
            let l1 = (x1 - x0).pow(2) + (y1 - y0).pow(2);
            if l < l1 {
                j = i1;
                l = l1;
            }
        }
        println!("{}", j + 1);
    }
}
