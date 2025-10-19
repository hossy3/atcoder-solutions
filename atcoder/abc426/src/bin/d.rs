use proconio::{input, marker::Chars};

// 0000 や 111 といったもっとも長いかたまりを返す
fn seq_len(s: &[char], c: char) -> usize {
    let mut count = 0;
    let mut result = 0;
    for &c0 in s {
        if c0 == c {
            count += 1;
            result = result.max(count);
        } else {
            count = 0;
        }
    }
    result
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            _n: usize,
            s: Chars,
        }

        // 右端・左端とも、「端のまま記号を変える」「一番長いかたまりに混ぜる」の二択で行動する
        // 01001 まんなかの 00 が長いかたまり
        // 11001
        // 10001
        // 00001
        // 00000
        let count0 = s.iter().filter(|&&c| c == '0').count();
        let count1 = s.iter().filter(|&&c| c == '1').count();
        let seq0 = seq_len(&s, '0');
        let seq1 = seq_len(&s, '1');
        let result0 = (count0 - seq0) * 2 + count1; // 全部 0 にする
        let result1 = (count1 - seq1) * 2 + count0; // 全部 1 にする
        let result = result0.min(result1);
        println!("{result}");
    }
}
