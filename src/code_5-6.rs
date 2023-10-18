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

fn rec(i: usize, h: &Vec<i64>, dp: &mut Vec<i64>) -> i64 {
    if dp[i] < INF {
        dp[i]
    } else if i == 0 {
        0
    } else {
        let mut res = INF;

        chmin(&mut res, &(rec(i - 1, h, dp) + (h[i] - h[i - 1]).abs()));

        if i > 1 {
            chmin(&mut res, &(rec(i - 2, h, dp) + (h[i] - h[i - 2]).abs()));
        }

        dp[i] = res;
        res
    }
}

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }

    let mut dp = vec![INF; n];

    rec(n - 1, &h, &mut dp);

    println!("{}", dp[n - 1]);
}
