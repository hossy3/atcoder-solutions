use proconio::input;

fn main() {
    input! {
        n: usize,
        qn: usize,
        xy: [(i64, i64); n],
        q: [usize; qn],
    }
    let mut add_min = 2 << 60;
    let mut add_max = -(2 << 60);
    let mut sub_min = 2 << 60;
    let mut sub_max = -(2 << 60);
    for (x, y) in &xy {
        let add = x + y;
        let sub = x - y;
        add_max = add_max.max(add);
        add_min = add_min.min(add);
        sub_max = sub_max.max(sub);
        sub_min = sub_min.min(sub);
    }
    for i in &q {
        let (x, y) = xy[*i - 1];
        let add = x + y;
        let sub = x - y;
        let a = [add_max - add, add - add_min, sub_max - sub, sub - sub_min];
        let score = a.iter().max();
        println!("{}", score.unwrap());
    }
}
