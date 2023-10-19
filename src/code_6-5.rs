use itertools::izip;
use proconio::input;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        hs: [(usize, usize); n],
    }

    let h: Vec<_> = hs.iter().map(|k| k.0).collect();
    let s: Vec<_> = hs.iter().map(|k| k.1).collect();

    let mut left = 0;
    let mut right = INF;

    while right - left > 1 {
        let mid = left + (right - left) / 2;

        let mut ok = true;
        let mut t = vec![0; n];

        for (ti, hi, si) in izip!(&mut t, &h, &s) {
            if mid < *hi {
                ok = false;
            } else {
                *ti = (mid - *hi) / *si;
            }
        }

        t.sort();

        for (i, ti) in t.iter().enumerate() {
            if *ti < i {
                ok = false;
            }
        }

        if ok {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}
