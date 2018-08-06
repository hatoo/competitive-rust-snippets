#[snippet = "mod"]
#[allow(dead_code)]
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet = "mod"]
#[allow(dead_code)]
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[snippet = "mod"]
#[allow(dead_code)]
/// (gcd, x, y)
pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x, y) = extgcd(b, a % b);
        (gcd, y, x - (a / b) * y)
    }
}

#[snippet = "mod"]
#[allow(dead_code)]
/// x ^ n % m
pub fn mod_pow(x: u64, n: u64, m: u64) -> u64 {
    let mut res = 1;
    let mut x = x % m;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % m;
        }
        x = (x * x) % m;
        n >>= 1;
    }
    res
}

#[snippet = "mod"]
#[allow(dead_code)]
pub fn mod_inverse(a: u64, m: u64) -> u64 {
    let (_, x, _) = extgcd(a as i64, m as i64);
    ((m as i64 + x) as u64 % m) % m
}

#[snippet = "mod"]
#[allow(dead_code)]
pub fn fact_table(len: usize, m: u64) -> Vec<u64> {
    let mut res = vec![1; len + 1];
    for i in 1..len + 1 {
        res[i] = (i as u64 * res[i - 1]) % m;
    }
    res
}

#[snippet = "mod"]
#[allow(dead_code)]
/// Factorial and Inverse factorial table
pub fn fact_inv_table(size: usize, m: u64) -> (Vec<u64>, Vec<u64>) {
    let mut fact = vec![1; size];
    let mut fact_inv = vec![1; size];

    for i in 2..size {
        fact[i] = fact[i - 1] * i as u64 % m;
        fact_inv[i] = m - ((m / i as u64) * fact_inv[(m % i as u64) as usize] % m);
    }

    for i in 1..size {
        fact_inv[i] = fact_inv[i - 1] * fact_inv[i] % m;
    }

    (fact, fact_inv)
}

#[snippet = "mod"]
#[allow(dead_code)]
/// (a mod p, e when n! = a p\^e)
pub fn mod_fact(n: u64, p: u64, fact: &[u64]) -> (u64, u64) {
    if n == 0 {
        (1, 0)
    } else {
        let (a, b) = mod_fact(n / p, p, fact);
        let pow = b + n / p;

        if n / p % 2 != 0 {
            (a * (p - fact[(n % p) as usize]) % p, pow)
        } else {
            (a * fact[(n % p) as usize] % p, pow)
        }
    }
}

#[snippet = "mod"]
#[allow(dead_code)]
/// C(n, k) % p
pub fn mod_comb(n: u64, k: u64, p: u64, fact: &[u64]) -> u64 {
    if n < k {
        0
    } else {
        let (a1, e1) = mod_fact(n, p, fact);
        let (a2, e2) = mod_fact(k, p, fact);
        let (a3, e3) = mod_fact(n - k, p, fact);

        if e1 > e2 + e3 {
            0
        } else {
            a1 * mod_inverse(a2 * a3 % p, p) % p
        }
    }
}

#[snippet = "mod"]
#[allow(dead_code)]
/// H(n, k) % p
pub fn mod_comb_repetition(n: u64, k: u64, p: u64, fact: &[u64]) -> u64 {
    mod_comb(n - 1 + k, n - 1, p, fact)
}

#[snippet]
#[snippet = "fib"]
#[allow(dead_code)]
/// Matrix mul
fn mat_mul2(x: &[u64; 4], y: &[u64; 4], p: u64) -> [u64; 4] {
    [
        (x[0] * y[0] + x[1] * y[2]) % p,
        (x[0] * y[1] + x[1] * y[3]) % p,
        (x[2] * y[0] + x[3] * y[2]) % p,
        (x[2] * y[1] + x[3] * y[3]) % p,
    ]
}

#[snippet]
#[snippet = "fib"]
#[allow(dead_code)]
/// Matrix pow
fn pow_mat(m: &[u64; 4], n: u64, p: u64) -> [u64; 4] {
    let mut m = m.clone();
    let mut res = [1, 0, 1, 0];

    for i in 0.. {
        if n >> i == 0 {
            break;
        }

        if n >> i & 1 == 1 {
            res = mat_mul2(&res, &m, p);
        }
        m = mat_mul2(&m, &m, p);
    }

    res
}

#[snippet = "fib"]
#[allow(dead_code)]
/// Fast fibonacci calculation
fn fib(i: u64, p: u64) -> u64 {
    let m = pow_mat(&[1, 1, 1, 0], i, p);
    m[1]
}

#[test]
fn test_mod_comb_repetition() {
    let m = 1_000_000_007;
    let fact = fact_table(200000, m);

    assert_eq!(mod_comb_repetition(10, 2, m, &fact), 55);
    assert_eq!(mod_comb_repetition(10, 3, m, &fact), 220);
    assert_eq!(mod_comb_repetition(10, 4, m, &fact), 715);
    assert_eq!(mod_comb_repetition(400, 296, m, &fact), 546898535);
    assert_eq!(mod_comb_repetition(100000, 100000, m, &fact), 939733670);
}

#[test]
fn test_fact_inv_table() {
    let m = 1_000_000_007;
    let size = 1000;
    let (fact, fact_inv) = fact_inv_table(1000, m);

    for i in 0..size {
        assert_eq!(fact[i] * fact_inv[i] % m, 1);
    }
}

#[test]
fn test_mod_pow() {
    let m = 1_000_000_007;
    let x = 1234;
    let mut t = 1;
    for i in 0..1000 {
        assert_eq!(mod_pow(x, i, m), t);
        t = t * x % m;
    }
}

#[test]
fn test_fib() {
    pub const M: u64 = 1_000_000_007;
    assert_eq!(fib(0, M), 0);
    assert_eq!(fib(1, M), 1);
    assert_eq!(fib(2, M), 1);
    assert_eq!(fib(3, M), 2);
    assert_eq!(fib(4, M), 3);
    assert_eq!(fib(5, M), 5);
    assert_eq!(fib(6, M), 8);
    assert_eq!(fib(7, M), 13);
    assert_eq!(fib(8, M), 21);

    assert_eq!(fib(10000000000, M) * fib(10000000001, M) % M, 128493982);
}
