use proconio::{input, marker::Chars};

fn chmin<T>(a: &mut T, b: &T)
where
    T: Clone + Copy + Ord + PartialOrd,
{
    if *a > *b {
        *a = *b;
    }
}

const INF: usize = 1 << 29;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut dp = vec![vec![INF; t.len() + 1]; s.len() + 1];

    dp[0][0] = 0;

    for i in 0..=s.len() {
        for j in 0..=t.len() {
            // 変更操作
            if i > 0 && j > 0 {
                let prev = dp[i - 1][j - 1];

                if s[i - 1] == t[j - 1] {
                    chmin(&mut dp[i][j], &prev);
                } else {
                    chmin(&mut dp[i][j], &(prev + 1));
                }
            }

            // 削除操作
            if i > 0 {
                let prev = dp[i - 1][j];

                chmin(&mut dp[i][j], &(prev + 1));
            }

            //　挿入操作
            if j > 0 {
                let prev = dp[i][j - 1];

                chmin(&mut dp[i][j], &(prev + 1));
            }
        }
    }

    println!("{}", dp[s.len()][t.len()]);
}
