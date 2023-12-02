use proconio::input;
use std::cmp::min;

const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        abw: [(usize, usize, i64); m],
    }

    let mut dp = vec![vec![INF; n]; n];

    for (a, b, w) in abw.iter() {
        dp[*a][*b] = *w;
    }

    for v in 0..n {
        dp[v][v] = 0;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
            }
        }
    }

    let mut exist_negative_cycle = false;

    for v in 0..n {
        if dp[v][v] < 0 {
            exist_negative_cycle = true;
        }
    }

    if exist_negative_cycle {
        println!("NEGATIVE CYCLE");
    } else {
        for dp_i in dp.iter() {
            for (j, dp_i_j) in dp_i.iter().enumerate() {
                if j > 0 {
                    print!(" ");
                }
                if *dp_i_j < INF / 2 {
                    print!("{}", *dp_i_j);
                } else {
                    print!("INF");
                }

                println!();
            }
        }
    }
}
