use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }

    let minimum_dist: f64 = xy
        .iter()
        .combinations(2)
        .map(|xy| calc_dist(xy[0].0, xy[0].1, xy[1].0, xy[1].1))
        .min_by(f64::total_cmp)
        .unwrap_or(1000000000.0);

    println!("{}", minimum_dist);
}

fn calc_dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt()
}
