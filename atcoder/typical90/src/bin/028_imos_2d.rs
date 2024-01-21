use proconio::input;

fn main() {
    input! {
        n: usize,
        rect: [((usize, usize), (usize, usize)); n],
    }

    const M: usize = 1000;
    let mut imos = vec![vec![0i64; M + 1]; M + 1];
    for ((lx, ly), (rx, ry)) in rect {
        imos[ly][lx] += 1;
        imos[ly][rx] += -1;
        imos[ry][lx] += -1;
        imos[ry][rx] += 1;
    }

    for y in 0..(M + 1) {
        for x in 0..M {
            imos[y][x + 1] += imos[y][x];
        }
    }
    for y in 0..M {
        for x in 0..(M + 1) {
            imos[y + 1][x] += imos[y][x];
        }
    }

    let mut count = vec![0usize; n + 1];
    for y in 0..(M + 1) {
        for x in 0..(M + 1) {
            count[imos[y][x] as usize] += 1;
        }
    }

    for &x in count[1..].iter() {
        println!("{x}");
    }
}
