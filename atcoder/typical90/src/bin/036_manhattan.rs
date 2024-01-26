use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        qn: usize,
        xy: [(i64, i64); n],
        q: [Usize1; qn],
    }

    let mut add_min = i64::MAX;
    let mut add_max = i64::MIN;
    let mut sub_min = i64::MAX;
    let mut sub_max = i64::MIN;
    for (x, y) in &xy {
        let add = x + y;
        let sub = x - y;
        add_max = add_max.max(add);
        add_min = add_min.min(add);
        sub_max = sub_max.max(sub);
        sub_min = sub_min.min(sub);
    }

    for i in q {
        let (x, y) = xy[i];
        let add = x + y;
        let sub = x - y;
        let a = [add_max - add, add - add_min, sub_max - sub, sub - sub_min];
        let result = a.iter().max().unwrap();
        println!("{result}");
    }
}
