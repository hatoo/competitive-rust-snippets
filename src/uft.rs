#[snippet = "UFT"]
#[allow(dead_code)]
/// Union Find Tree
pub struct UFT {
    pub par: Vec<usize>,
    pub rank: Vec<usize>,
}

#[snippet = "UFT"]
impl UFT {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        UFT {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    #[allow(dead_code)]
    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let p = self.par[x];
            let pp = self.root(p);
            self.par[x] = pp;
            pp
        }
    }

    #[allow(dead_code)]
    pub fn merge(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);
        if x == y {
            return;
        }

        if self.rank[x] < self.rank[y] {
            self.par[x] = y;
        } else {
            self.par[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
}

use std;

#[snippet = "WeightedUFT"]
/// https://qiita.com/drken/items/cce6fc5c579051e64fab
pub struct WeightedUFT {
    pub par: Vec<usize>,
    pub rank: Vec<usize>,
    pub diff_weight: Vec<i64>,
}

#[snippet = "WeightedUFT"]
impl WeightedUFT {
    pub fn new(size: usize) -> WeightedUFT {
        WeightedUFT {
            par: (0..size).collect(),
            rank: vec![0; size],
            diff_weight: vec![0; size],
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let p = self.par[x];
            let r = self.root(p);
            self.diff_weight[x] += self.diff_weight[p];
            self.par[x] = r;
            r
        }
    }

    pub fn weight(&mut self, x: usize) -> i64 {
        self.root(x);
        self.diff_weight[x]
    }

    pub fn merge(&mut self, mut x: usize, mut y: usize, mut w: i64) -> bool {
        w += self.weight(x);
        w -= self.weight(y);

        x = self.root(x);
        y = self.root(y);

        if x == y {
            return false;
        }

        if self.rank[x] < self.rank[y] {
            std::mem::swap(&mut x, &mut y);
            w = -w;
        }

        if self.rank[x] == self.rank[y] {
            self.rank[y] += 1;
        }
        self.par[y] = x;
        self.diff_weight[y] = w;
        true
    }
}
