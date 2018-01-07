#![snippet = "mod"]

#[allow(dead_code)]
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}


#[allow(dead_code)]
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

// (gcd, x, y)
#[allow(dead_code)]
fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x, y) = extgcd(b, a % b);
        (gcd, y, x - (a / b) * y)
    }
}

#[allow(dead_code)]
fn mod_pow(x: u64, n: u64, m: u64) -> u64 {
    let mut res = 1;
    let mut x = x;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % m;
        }
        x = (x * x) % m;
        n = n >> 1;
    }
    res
}

#[allow(dead_code)]
fn mod_inverse(a: u64, m: u64) -> u64 {
    let (_, x, _) = extgcd(a as i64, m as i64);
    ((m as i64 + x) as u64 % m) % m
}

#[allow(dead_code)]
fn fact_table(len: usize, m: u64) -> Vec<u64> {
    let mut res = vec![1; len];
    for i in 1..len {
        res[i] = (i as u64 * res[i - 1]) % m;
    }
    res
}


// (a mod p, e when n! = a p^e)
#[allow(dead_code)]
fn mod_fact(n: u64, p: u64, fact: &Vec<u64>) -> (u64, u64) {
    if n == 0 {
        (1, 0)
    } else {
        let (a, b) = mod_fact(n / p, p, fact);
        let e = b + n / p;

        if n / p % 2 != 0 {
            (a * (p - fact[(n % p) as usize]) % p, e)
        } else {
            (a * fact[(n % p) as usize] % p, e)
        }
    }
}

#[allow(dead_code)]
fn mod_comb(n: u64, k: u64, p: u64, fact: &Vec<u64>) -> u64 {
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
