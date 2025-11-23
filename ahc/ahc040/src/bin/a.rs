use proconio::input_interactive;

fn main() {
    input_interactive! {
        n: usize,
        t: usize,
        _sigma: i64,
        wh: [(i64, i64); n],
    }

    // 総面積を求める
    let area_sum: i64 = wh.iter().map(|&(w, h)| w * h).sum();
    let square_len = (area_sum as f64).sqrt() as i64;

    for i in 0..t {
        // 目標高さを理想的な正方形の0.75～1.5倍ほどで案分する
        let max_h = square_len * (t * 3 + i * 3) as i64 / (t * 4) as i64;

        println!("{n}"); // 全部使う

        let mut base = -1;

        let mut boxes = vec![];
        let mut height_sum = 0i64;
        let mut last_spurt = false;

        for (i, &(w, h)) in wh.iter().enumerate() {
            let w = w.clamp(20000, 100000);
            let h = h.clamp(20000, 100000);
            
            // 縦長にする
            let r = if w > h { 0 } else { 1 };
            let (w, h) = (w.max(h), w.min(h));

            // ラストスパート: 残りすべて縦長にしても目標高さ以内なら、縦長にして縦に積み上げる
            if !last_spurt {
                let rest_height_sum: i64 = wh[i..n].iter().map(|&(w, h)| w.max(h)).sum();
                if height_sum + rest_height_sum <= max_h {
                    last_spurt = true;
                }
            }

            if last_spurt {
                let r = 1 - r;
                println!("{i} {r} L {base}");
                base = i as i64;
                continue;
            }

            // 横長の四角を縦に積み上げるとき
            boxes.push((i, w, h));
            height_sum += h;

            if boxes.len() == 1 || height_sum <= max_h {
                println!("{i} {r} L {base}");
                base = i as i64;
                continue;
            }

            // 次の段に移動する
            boxes.clear();

            // ラストスパート判定 次の段に移動した直後にも行う
            let rest_height_sum: i64 = wh[i..n].iter().map(|&(w, h)| w.max(h)).sum();
            if rest_height_sum <= max_h {
                last_spurt = true;
            }

            if last_spurt {
                let (r, w, h) = (1 - r, h, w);
                boxes.push((i, w, h));
                height_sum = h;
                println!("{i} {r} L -1");
                base = i as i64;
                continue;
            }

            boxes.push((i, w, h));
            height_sum = h;
            println!("{i} {r} L -1");
            base = i as i64;
        }

        input_interactive! {
            w: i64,
            h: i64,
        }
        println!("# {w} {h}");
    }
}
