#[snippet = "Manacher"]
#[allow(dead_code)]
struct Manacher {
    r: Vec<usize>,
}

#[snippet = "Manacher"]
impl Manacher {
    #[allow(dead_code)]
    fn new<T: Eq>(seq: &[T]) -> Manacher {
        let mut r = vec![0; 2 * seq.len() - 1];
        let mut i = 0;
        let mut j = 0;
        while i < r.len() {
            while i >= j && i + j < r.len() && Self::get(seq, i - j) == Self::get(seq, i + j) {
                j += 1;
            }
            r[i] = j;
            let mut k = 1;
            while i >= k && i + k < r.len() && k + r[i - k] < j {
                r[i + k] = r[i - k];
                k += 1;
            }
            i += k;
            j -= k;
        }

        Manacher { r: r }
    }

    // [l,r]
    #[allow(dead_code)]
    fn is_palindrome(&self, l: usize, r: usize) -> bool {
        self.r[l + r] >= r - l + 1
    }

    #[allow(dead_code)]
    fn get<T: Eq>(s: &[T], i: usize) -> Option<&T> {
        if i & 1 == 1 {
            None
        } else {
            s.get(i >> 1)
        }
    }
}

#[test]
fn test_manacher() {
    let seq: Vec<char> = "abcbakjkj".chars().collect();
    let manacher = Manacher::new(&seq);

    assert_eq!(manacher.is_palindrome(0, 4), true);
    assert_eq!(manacher.is_palindrome(0, 3), false);
}
