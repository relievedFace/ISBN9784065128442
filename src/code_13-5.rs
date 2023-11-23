use proconio::input;

type Graph = Vec<Vec<usize>>;

fn dfs(g: &Graph, v: usize, cur: i32, color: &mut Vec<i32>) -> bool {
    color[v] = cur;

    for next_v in g[v].iter() {
        if color[*next_v] != -1 {
            if color[*next_v] == cur {
                return false;
            }

            continue;
        }

        if !dfs(g, *next_v, 1 - cur, color) {
            return false;
        }
    }

    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut g = vec![vec![]; n];

    for (a, b) in ab.iter() {
        g[*a].push(*b);
        g[*b].push(*a);
    }

    let mut color = vec![-1; n];
    let mut is_bipartite = true;

    for v in 0..n {
        if color[v] != -1 {
            continue;
        }

        if !dfs(&g, v, 0, &mut color) {
            is_bipartite = false;
        }
    }

    if is_bipartite {
        println!("Yes");
    } else {
        println!("No");
    }
}
