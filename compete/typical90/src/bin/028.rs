use proconio::input;

fn main() {
    input! {
        n: usize,
        rect: [((usize, usize), (usize, usize)); n],
    }
    let mut imos = vec![vec![0i64; 1001]; 1001];
    for ((lx, ly), (rx, ry)) in rect {
        imos[ly][lx] += 1;
        imos[ly][rx] += -1;
        imos[ry][lx] += -1;
        imos[ry][rx] += 1;
    }
    for y in 0..1001 {
        for x in 0..1000 {
            imos[y][x + 1] += imos[y][x];
        }
    }
    for y in 0..1000 {
        for x in 0..1001 {
            imos[y + 1][x] += imos[y][x];
        }
    }
    let mut count = vec![0i64; n + 1];
    for y in 0..1001 {
        for x in 0..1001 {
            count[imos[y][x] as usize] += 1;
        }
    }
    for i in 1..=n {
        println!("{}", count[i]);
    }
}
