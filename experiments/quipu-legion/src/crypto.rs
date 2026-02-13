use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, Zero, ToPrimitive};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct PublicKey {
    pub n: BigUint,
    pub e: BigUint,
}

#[derive(Debug, Clone)]
pub struct PrivateKey {
    pub n: BigUint,
    pub d: BigUint,
}

pub struct KeyPair {
    pub public: PublicKey,
    pub private: PrivateKey,
}

pub fn encrypt(m: &BigUint, key: &PublicKey) -> BigUint {
    m.modpow(&key.e, &key.n)
}

pub fn decrypt(c: &BigUint, key: &PrivateKey) -> BigUint {
    c.modpow(&key.d, &key.n)
}

pub fn generate_keypair(bits: usize) -> KeyPair {
    let mut rng = rand::thread_rng();
    let p = gen_prime(bits / 2, &mut rng);
    let q = gen_prime(bits / 2, &mut rng);

    let n = &p * &q;
    let phi = (&p - 1u32) * (&q - 1u32);
    let e = BigUint::from(65537u32);

    // Calculate d = e^-1 mod phi
    // num_bigint doesn't support modinverse easily without checking features or implementing ext_gcd.
    // Let's implement extended euclidean algorithm for BigUint.
    // Since we are dealing with unsigned, we need to be careful with negative coefficients.
    // d * e = 1 (mod phi) -> d * e + k * phi = 1.
    // We want d > 0.

    let d = mod_inverse(&e, &phi).expect("Inverse should exist for valid primes");

    KeyPair {
        public: PublicKey { n: n.clone(), e },
        private: PrivateKey { n, d },
    }
}

fn gen_prime<R: Rng + ?Sized>(bits: usize, rng: &mut R) -> BigUint {
    loop {
        let n = rng.gen_biguint(bits as u64);
        // Ensure it's odd
        let n = if n.bit(0) { n } else { n + 1u32 };
        if is_prime(&n, 10) {
            return n;
        }
    }
}

// Miller-Rabin primality test
fn is_prime(n: &BigUint, k: usize) -> bool {
    if *n <= BigUint::from(1u32) { return false; }
    if *n <= BigUint::from(3u32) { return true; }
    if n % 2u32 == BigUint::zero() { return false; }

    // Write n-1 as 2^r * d
    let one = BigUint::one();
    let n_minus_one = n - &one;
    let mut d = n_minus_one.clone();
    let mut r = 0;

    let two = BigUint::from(2u32);

    while &d % &two == BigUint::zero() {
        d /= &two;
        r += 1;
    }

    let mut rng = rand::thread_rng();

    'witness: for _ in 0..k {
        // Pick random a in [2, n-2]
        let a = rng.gen_biguint_range(&two, &(n - &two));
        let mut x = a.modpow(&d, n);

        if x == one || x == n_minus_one {
            continue;
        }

        for _ in 0..r-1 {
            x = x.modpow(&two, n);
            if x == n_minus_one {
                continue 'witness;
            }
        }
        return false; // Composite
    }

    true // Probably prime
}

// Extended Euclidean Algorithm to find modular inverse
// Returns x such that a*x = 1 (mod m)
fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    // using converting to signed logic or iterative unsigned
    // a * x = 1 mod m

    let mut t = BigUint::zero();
    let mut newt = BigUint::one();
    let mut r = m.clone();
    let mut newr = a.clone();

    // We need to track signs because the standard algorithm uses negatives.
    // Instead of full signed BigInt (which might not be pulled in), we can track if t is negative.
    // t_sign: true if positive, false if negative.
    let mut t_sign = true;
    let mut newt_sign = true;

    while !newr.is_zero() {
        let quotient = &r / &newr;

        // (t, newt) = (newt, t - quotient * newt)
        let temp_t = t.clone();
        let temp_t_sign = t_sign;

        t = newt.clone();
        t_sign = newt_sign;

        // Calculate newt = temp_t - quotient * newt
        // We need to handle signs manually
        let q_times_newt = &quotient * &newt;

        if temp_t_sign == newt_sign {
            // signs match: subtract magnitudes
            // if temp_t >= q*newt, result is positive (same sign)
            // else result is negative (flip sign)
            if temp_t >= q_times_newt {
                newt = temp_t - q_times_newt;
                newt_sign = temp_t_sign;
            } else {
                newt = q_times_newt - temp_t;
                newt_sign = !temp_t_sign;
            }
        } else {
            // signs different: effectively addition
            // t - (-q*newt) = t + q*newt
            newt = temp_t + q_times_newt;
            newt_sign = temp_t_sign; // Keep sign of first term (conceptually)
            // Wait: t (pos) - (-val) = pos + val = pos.
            // t (neg) - (val) = neg - val = neg.
            // So sign matches temp_t_sign.
        }

        // (r, newr) = (newr, r - quotient * newr) -> Standard Euclidean for r (always positive)
        let temp_r = r;
        r = newr;
        newr = temp_r - &quotient * &r;
    }

    if r > BigUint::one() {
        return None; // Not invertible
    }

    if t_sign {
        Some(t)
    } else {
        // Result is negative t. In mod m, this is m - t.
        Some(m - t)
    }
}
