use proconio::input;

fn chmin<T>(a: &mut T, b: &T)
where
    T: Clone + Copy + Ord + PartialOrd,
{
    if *a > *b {
        *a = *b;
    }
}

const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }

    let mut dp = vec![INF; n];

    dp[0] = 0;

    for i in 0..n {
        let prev = dp[i];

        if i + 1 < n {
            chmin(&mut dp[i + 1], &(prev + (h[i] - h[i + 1]).abs()));
        }

        if i + 2 < n {
            chmin(&mut dp[i + 2], &(prev + (h[i] - h[i + 2]).abs()));
        }
    }

    println!("{}", dp[n - 1]);
}
