use proconio::input;

const INF: i64 = 1 << 60;

#[derive(Clone)]
struct Edge {
    to: usize,
    w: i64,
}

impl Edge {
    fn new(to: usize, w: i64) -> Self {
        Edge { to, w }
    }
}

type _Graph = Vec<Vec<Edge>>;

fn chmin<T: Clone + Copy + Ord>(a: &mut T, b: &T) -> bool {
    if *a > *b {
        *a = *b;
        true
    } else {
        false
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        abw: [(usize, usize, i64); m],
    }

    let mut g = vec![vec![]; n];

    for (a, b, w) in abw.iter() {
        g[*a].push(Edge::new(*b, *w));
    }

    let mut used = vec![false; n];
    let mut dist = vec![INF; n];
    dist[s] = 0;

    for _ in 0..n {
        let mut min_dist = INF;
        let mut min_v = None;

        for v in 0..n {
            if !used[v] && dist[v] < min_dist {
                min_dist = dist[v];
                min_v = Some(v);
            }
        }

        if let Some(min_v) = min_v {
            for e in g[min_v].iter() {
                let dvw = dist[e.to] + e.w;

                chmin(&mut dist[e.to], &dvw);
            }

            used[min_v] = true;
        } else {
            break;
        }
    }

    for dist_v in dist.iter() {
        if *dist_v < INF {
            println!("{}", *dist_v);
        } else {
            println!("INF");
        }
    }
}
