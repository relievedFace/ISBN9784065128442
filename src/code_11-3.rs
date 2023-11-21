struct UnionFind {
    par: Vec<Option<usize>>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: vec![None; n],
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if let Some(par_x) = self.par[x] {
            self.par[x] = Some(self.root(par_x));
            self.par[x].unwrap()
        } else {
            x
        }
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unit(&mut self, x: usize, y: usize) -> bool {
        let x = self.root(x);
        let y = self.root(y);

        if x == y {
            return false;
        }

        if self.siz[x] < self.siz[y] {
            self.siz.swap(x, y);
        }

        self.par[y] = Some(x);
        self.siz[x] += self.siz[y];
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.siz[x]
    }
}

fn main() {
    let mut uf = UnionFind::new(7);

    uf.unit(1, 2);
    uf.unit(2, 3);
    uf.unit(5, 6);
    println!("{}", uf.is_same(1, 3));
    println!("{}", uf.is_same(2, 5));

    uf.unit(1, 6);
    println!("{}", uf.is_same(2, 5));
}
