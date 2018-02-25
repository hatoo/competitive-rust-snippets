use std;

#[snippet = "Vector2D"]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub struct Vector2D(f64, f64);

#[snippet = "Vector2D"]
impl Vector2D {
    const EPS: f64 = 1e-10;
    pub fn add(a: f64, b: f64) -> f64 {
        let c = a + b;
        if c.abs() < Self::EPS {
            0.0
        } else {
            c
        }
    }

    pub fn dot(self, other: Vector2D) -> f64 {
        Self::add(self.0 * other.0, self.1 * other.1)
    }

    pub fn det(self, other: Vector2D) -> f64 {
        Self::add(self.0 * other.1, -self.1 * other.0)
    }
    pub fn len(&self) -> f64 {
        f64::sqrt((self.0).powi(2) + (self.1).powi(2))
    }
    pub fn unit(self) -> Vector2D {
        let l = self.len();
        Vector2D(self.0 / l, self.1 / l)
    }
    pub fn normal(self) -> Vector2D {
        Vector2D(self.1, -self.0)
    }
}

#[snippet = "Vector2D"]
impl std::ops::Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D(Vector2D::add(self.0, rhs.0), Vector2D::add(self.1, rhs.1))
    }
}

#[snippet = "Vector2D"]
impl std::ops::Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        Vector2D(Vector2D::add(self.0, -rhs.0), Vector2D::add(self.1, -rhs.1))
    }
}

#[snippet = "Vector2D"]
impl std::ops::Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2D(rhs * self.0, rhs * self.1)
    }
}

#[snippet = "Vector2D"]
impl std::ops::Div<f64> for Vector2D {
    type Output = Vector2D;

    fn div(self, rhs: f64) -> Self::Output {
        Vector2D(self.0 / rhs, self.1 / rhs)
    }
}

use total::Total;

#[snippet = "convex_hull"]
#[allow(dead_code)]
fn convex_hull(vs: &[Vector2D]) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..vs.len()).collect();
    idx.sort_by_key(|&i| Total((vs[i].0, vs[i].1)));

    let mut res = Vec::new();

    for &i in &idx {
        while res.len() > 1
            && Vector2D::det(
                vs[res[res.len() - 1]] - vs[res[res.len() - 2]],
                vs[i] - vs[res[res.len() - 1]],
            ) <= 0.0
        {
            res.pop();
        }
        res.push(i);
    }
    let t = res.len();

    for &i in idx.iter().rev().skip(1) {
        while res.len() > t
            && (vs[res[res.len() - 1]] - vs[res[res.len() - 2]]).det(vs[i] - vs[res[res.len() - 1]])
                <= 0.0
        {
            res.pop();
        }
        res.push(i);
    }

    res.pop();
    res
}

#[test]
fn test_convex_hull() {
    let vs = vec![
        Vector2D(-1.0, -1.0),
        Vector2D(-1.0, 1.0),
        Vector2D(1.0, 1.0),
        Vector2D(1.0, -1.0),
        Vector2D(0.0, 0.0),
        Vector2D(0.1, 0.1),
    ];

    let mut idx = convex_hull(&vs);
    idx.sort();

    assert_eq!(&idx, &[0, 1, 2, 3]);
}
