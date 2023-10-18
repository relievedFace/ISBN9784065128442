use proconio::input;

fn chmax<T>(a: &mut T, b: &T)
where
    T: Clone + Copy + Ord + PartialOrd,
{
    if *a < *b {
        *a = *b;
    }
}

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }

    let weight: Vec<_> = wv.iter().map(|k| k.0).collect();
    let value: Vec<_> = wv.iter().map(|k| k.1).collect();

    let mut dp = vec![vec![0; w + 1]; n + 1];

    for i in 0..n {
        for w in 0..=w {
            // `w - weight[i] >= 0`の代わり
            if w.checked_sub(weight[i]).is_some() {
                let prev = dp[i][w - weight[i]];

                chmax(&mut dp[i + 1][w], &(prev + value[i]));
            }

            let prev = dp[i][w];

            chmax(&mut dp[i + 1][w], &prev);
        }
    }

    println!("{}", dp[n][w]);
}
