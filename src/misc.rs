use std;

#[snippet = "M"]
#[allow(dead_code)]
pub const M: u64 = 1_000_000_007;

#[snippet = "INF"]
#[allow(dead_code)]
pub const INF: u64 = 1 << 60;

#[snippet = "CmpBy"]
pub struct CmpBy<T, U>(T, U);
#[snippet = "CmpBy"]
impl<T: PartialEq, U> PartialEq for CmpBy<T, U> {
    fn eq(&self, other: &Self) -> bool {
        (self.0).eq(&other.0)
    }
}
#[snippet = "CmpBy"]
impl<T: Eq, U> Eq for CmpBy<T, U> {}
#[snippet = "CmpBy"]
impl<T: PartialOrd, U> PartialOrd for CmpBy<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.0).partial_cmp(&other.0)
    }
}
#[snippet = "CmpBy"]
impl<T: Ord, U> Ord for CmpBy<T, U> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0).cmp(&other.0)
    }
}

#[snippet = "adjacent4"]
#[allow(dead_code)]
fn adjacent4(x: usize, y: usize, sx: usize, sy: usize) -> Vec<(usize, usize)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|&(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < sx as isize && ny >= 0 && ny < sy as isize {
                Some((nx as usize, ny as usize))
            } else {
                None
            }
        })
        .collect()
}

#[snippet = "adjacent8"]
#[allow(dead_code)]
fn adjacent8(x: usize, y: usize, sx: usize, sy: usize) -> Vec<(usize, usize)> {
    [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ].into_iter()
        .filter_map(|&(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < sx as isize && ny >= 0 && ny < sy as isize {
                Some((nx as usize, ny as usize))
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn test_adjacent4() {
    let mut a4 = adjacent4(1, 1, 3, 3);
    let mut expected = [(0, 1), (2, 1), (1, 0), (1, 2)];
    a4.sort();
    expected.sort();

    assert_eq!(&a4, &expected);

    let mut a4 = adjacent4(0, 0, 3, 3);
    let mut expected = [(1, 0), (0, 1)];
    a4.sort();
    expected.sort();

    assert_eq!(&a4, &expected);
}

#[test]
fn test_adjacent8() {
    let mut a8 = adjacent8(1, 1, 3, 3);
    let mut expected = [
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 2),
        (2, 0),
        (2, 1),
        (2, 2),
    ];
    a8.sort();
    expected.sort();

    assert_eq!(&a8, &expected);

    let mut a8 = adjacent8(0, 0, 3, 3);
    let mut expected = [(1, 0), (0, 1), (1, 1)];
    a8.sort();
    expected.sort();

    assert_eq!(&a8, &expected);
}
