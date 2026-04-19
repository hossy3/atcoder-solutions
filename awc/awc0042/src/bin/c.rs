use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        d: usize,
        p: [usize; n],
        w: [usize; d],
    }

    let p_sum_inv = Mint::new(p.iter().sum::<usize>()).inv();
    let w_sum = Mint::new(w.iter().sum::<usize>());
    let result = p
        .iter()
        .fold(Mint::new(1), |acc, &p| acc * w_sum * p * p_sum_inv);
    println!("{result}");
}
