use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }
    let mut m = vec![vec![false; 100]; 100];
    for &(a, b, c, d) in &abcd {
        for i in a..b {
            for j in c..d {
                m[i][j] = true;
            }
        }
    }
    let result: usize = m.iter().map(|v| v.iter().filter(|&&x| x).count()).sum();
    println!("{result}");
}

//10     xxxxx
// 9     xxxxx
// 8     xxxxx
// 7     xxxxx
// 6     xxxxx
// 5     xxxxx
// 4xxx  xxxxx
// 3xxx  xxxxx
// 2xxx  xxxxx
// 1xxx  xxxxx
// 0xxx  xxxxx
//  01234567891
//            0
