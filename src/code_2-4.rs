use proconio::input;

fn calc_dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt()
}

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }

    let x: Vec<_> = xy.iter().map(|k| k.0).collect();
    let y: Vec<_> = xy.iter().map(|k| k.1).collect();

    let mut minimum_dist: f64 = 1000000000.0;

    for i in 0..n {
        for j in i + 1..n {
            let dist_i_j = calc_dist(x[i], y[i], x[j], y[j]);

            if dist_i_j < minimum_dist {
                minimum_dist = dist_i_j;
            }
        }
    }

    println!("{}", minimum_dist);
}
