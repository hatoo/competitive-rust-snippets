#[snippet = "UFT"]
#[allow(dead_code)]
struct UFT {
    pub par: Vec<usize>,
    pub rank: Vec<usize>,
}

#[snippet = "UFT"]
impl UFT {
    #[allow(dead_code)]
    fn new(n: usize) -> Self {
        UFT {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    #[allow(dead_code)]
    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let p = self.par[x];
            let pp = self.find(p);
            self.par[x] = pp;
            pp
        }
    }

    #[allow(dead_code)]
    fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
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
