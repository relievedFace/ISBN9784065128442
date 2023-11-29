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

    let mut exist_negative_cycle = false;
    let mut dist = vec![INF; n];
    dist[s] = 0;

    for iter in 0..n {
        let mut update = false;

        for v in 0..n {
            if dist[v] == INF {
                continue;
            }

            for e in g[v].iter() {
                let dvw = dist[e.to] + e.w;

                if chmin(&mut dist[e.to], &dvw) {
                    update = true;
                }
            }
        }

        if !update {
            break;
        }

        if iter == n - 1 && update {
            exist_negative_cycle = true;
        }
    }

    if exist_negative_cycle {
        println!("NEGATIVE CYCLE");
    } else {
        for v in dist.iter() {
            if *v < INF {
                println!("{}", *v);
            } else {
                println!("INF");
            }
        }
    }
}
