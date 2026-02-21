use crate::math::pow_mod;
use crate::roman::Roman;
use num_bigint::{BigUint, RandBigInt};
use num_integer::Integer;
use num_traits::{One, Zero}; // For gcd, extended_gcd?
                             // num-bigint doesn't implement Integer for BigUint directly in a way that gives extended_gcd easily for BigUint (signed return).
                             // Actually BigInt (signed) does.
use num_bigint::BigInt;

pub struct KeyPair {
    pub public: Roman,
    pub private: Roman,
    pub modulus: Roman,
}

pub fn generate_keys(bits: usize) -> KeyPair {
    let mut rng = rand::thread_rng();

    // Generate primes p, q
    let p = gen_prime(bits, &mut rng);
    let q = gen_prime(bits, &mut rng);

    let n = &p * &q;
    let one = BigUint::one();
    let p_minus_1 = &p - &one;
    let q_minus_1 = &q - &one;
    let phi = &p_minus_1 * &q_minus_1;

    let e = BigUint::from(65537u32);

    // Check gcd(e, phi) == 1
    if phi.gcd(&e) != one {
        // Unlucky, try again (recursive or loop)
        return generate_keys(bits);
    }

    // Calculate d = e^-1 mod phi
    // Use BigInt for extended Euclidean algorithm
    let e_signed = BigInt::from_biguint(num_bigint::Sign::Plus, e.clone());
    let phi_signed = BigInt::from_biguint(num_bigint::Sign::Plus, phi.clone());

    let extended_gcd = e_signed.extended_gcd(&phi_signed);
    // extended_gcd.x * e + extended_gcd.y * phi = gcd
    // we want x such that x*e = 1 mod phi
    let mut d_signed = extended_gcd.x;
    while d_signed < BigInt::zero() {
        d_signed += &phi_signed;
    }

    let d = d_signed.to_biguint().expect("d should be positive");

    KeyPair {
        public: Roman::from_biguint(e),
        private: Roman::from_biguint(d),
        modulus: Roman::from_biguint(n),
    }
}

fn gen_prime(bits: usize, rng: &mut impl RandBigInt) -> BigUint {
    loop {
        let n = rng.gen_biguint(bits as u64);
        if n.is_even() {
            continue;
        }
        if is_prime(&n) {
            return n;
        }
    }
}

fn is_prime(n: &BigUint) -> bool {
    // Miller-Rabin test
    if *n <= BigUint::from(3u32) {
        return *n > BigUint::one();
    }

    let n_minus_1 = n - 1u32;
    let mut d = n_minus_1.clone();
    let mut s = 0;
    while d.is_even() {
        d /= 2u32;
        s += 1;
    }

    // Probabilistic check with small bases
    let bases = [2u32, 3, 5, 7, 11, 13, 17, 19, 23];
    for b in bases {
        let b_uint = BigUint::from(b);
        if n == &b_uint {
            return true;
        }
        // If n < b, just checking b % n logic holds, but miller_rabin usually expects [2, n-2].
        // Given we generate large bits, n > 23 usually.
        if !miller_rabin_test(n, &n_minus_1, &d, s, &b_uint) {
            return false;
        }
    }
    true
}

fn miller_rabin_test(n: &BigUint, n_minus_1: &BigUint, d: &BigUint, s: u32, a: &BigUint) -> bool {
    let mut x = a.modpow(d, n);
    if x == BigUint::one() || x == *n_minus_1 {
        return true;
    }
    for _ in 0..s - 1 {
        x = x.modpow(&BigUint::from(2u32), n);
        if x == *n_minus_1 {
            return true;
        }
    }
    false
}

pub fn encrypt(msg: &Roman, key: &KeyPair) -> Roman {
    // C = M^e mod n
    pow_mod(msg, &key.public, &key.modulus)
}

pub fn decrypt(msg: &Roman, key: &KeyPair) -> Roman {
    // M = C^d mod n
    pow_mod(msg, &key.private, &key.modulus)
}
