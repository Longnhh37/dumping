//! Sieve of Eratosthenes: two ideas in one file.
//!
//! ============================================================================
//! IDEA 1: Classic sieve  ->  "is x prime?"
//! ============================================================================
//! Keep a table `is_prime[0..=n]`, initially all `true`.
//! Walk i = 2, 3, 4, ...
//!   - If `is_prime[i]` is still true, then i is prime
//!     (nobody smaller crossed it out, so it has no smaller divisor).
//!   - Cross out every multiple of i: i*i, i*i + i, i*i + 2i, ...
//!     Stop when i * i > n, because every composite c <= n has a prime factor
//!     <= sqrt(c) <= sqrt(n), so it was already crossed out by then.
//!
//! Why start at i*i? A multiple k*i with k < i has the smaller factor k,
//! so it was already crossed out when we processed the primes of k.
//!
//! Complexity: O(n log log n) time, O(n) memory.
//! Limitation: the table only says prime / not prime. It cannot tell you
//! *why* a number is composite.
//!
//! ============================================================================
//! IDEA 2: SPF sieve  ->  "factorize x quickly"
//! ============================================================================
//! Same sieve, but instead of a bool we store `spf[x]` =
//! the Smallest Prime Factor of x.
//!   - Initially spf[x] = x (every number starts as "its own smallest factor").
//!   - When prime i crosses out j, set spf[j] = i, but ONLY if spf[j] is
//!     still j. The first prime to touch j is the smallest one, so we never
//!     overwrite it.
//!     After that, x is prime  <=>  spf[x] == x.
//!
//! Factorizing x <= n:
//!   while x > 1: p = spf[x]; divide x by p as many times as possible;
//!   record (p, exponent).
//! Each division at least halves x, so this takes O(log x).
//! Example: 360 -> spf=2 -> 180 -> 90 -> 45 -> spf=3 -> 15 -> 5 -> spf=5 -> 1
//!          so 360 = 2^3 * 3^2 * 5.
//!
//! Trade-off: build costs O(n log log n) once, then EVERY query is O(log x).
//! Memory is 4 bytes per entry (u32) instead of 1 byte for bool.

/// Classic sieve. Returns `is_prime` where `is_prime[x]` tells if x is prime.
fn classic_sieve(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    // 0 and 1 are not prime by definition.
    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }

    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            // Cross out multiples of i, starting at i*i (see module docs).
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
        i += 1;
    }
    is_prime
}

/// Same as `classic_sieve`, but prints every step so you can watch it work.
fn classic_sieve_traced(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }

    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            let crossed: Vec<usize> = (i * i..=n).step_by(i).collect();
            println!("  prime {i}: cross out {crossed:?}");
            for &j in &crossed {
                is_prime[j] = false;
            }
        }
        i += 1;
    }
    println!("  stop: {i}*{i} = {} > {n}", i * i);
    is_prime
}

/// Builds the smallest-prime-factor table for 0..=n.
/// After this, `spf[x] == x` means x is prime (for x >= 2).
fn build_spf(n: usize) -> Vec<u32> {
    // Every number starts as its own smallest prime factor.
    let mut spf: Vec<u32> = (0..=n as u32).collect();

    let mut i = 2;
    while i * i <= n {
        // spf[i] == i means no smaller prime touched i, so i is prime.
        if spf[i] == i as u32 {
            for j in (i * i..=n).step_by(i) {
                // Only the FIRST prime to reach j is its smallest factor.
                if spf[j] == j as u32 {
                    spf[j] = i as u32;
                }
            }
        }
        i += 1;
    }
    spf
}

/// Factorizes x (1 <= x <= n) into (prime, exponent) pairs using the spf table.
/// Returns an empty vec for x = 1.
fn factorize(mut x: usize, spf: &[u32]) -> Vec<(u32, u32)> {
    assert!(x >= 1 && x < spf.len(), "x must be in 1..=n");

    let mut factors: Vec<(u32, u32)> = Vec::new();
    while x > 1 {
        let p = spf[x];
        let mut exp = 0;
        // Divide out p completely before moving to the next prime.
        while x.is_multiple_of(p as usize) {
            x /= p as usize;
            exp += 1;
        }
        factors.push((p, exp));
    }
    factors
}

/// Pretty-prints a factorization, e.g. [(2,3),(3,2),(5,1)] -> "2^3 * 3^2 * 5".
fn format_factors(factors: &[(u32, u32)]) -> String {
    if factors.is_empty() {
        return "1".to_string();
    }
    factors
        .iter()
        .map(|&(p, e)| {
            if e == 1 {
                p.to_string()
            } else {
                format!("{p}^{e}")
            }
        })
        .collect::<Vec<_>>()
        .join(" * ")
}

/// Same as `factorize`, but prints each division step.
fn factorize_traced(x: usize, spf: &[u32]) {
    let mut cur = x;
    print!("  {x}");
    while cur > 1 {
        let p = spf[cur] as usize;
        print!(" -> spf={p} -> {}", cur / p);
        cur /= p;
    }
    println!();
}

fn main() {
    // ---------------------------------------------------------------- Idea 1
    println!("=== IDEA 1: classic sieve (n = 30) ===");
    let is_prime = classic_sieve_traced(30);
    let primes: Vec<usize> = (0..is_prime.len()).filter(|&i| is_prime[i]).collect();
    println!("  primes: {primes:?}\n");

    // ---------------------------------------------------------------- Idea 2
    println!("=== IDEA 2: SPF sieve (n = 30) ===");
    let small = build_spf(30);
    println!("  spf table:");
    for x in 2..=30 {
        let tag = if small[x] as usize == x {
            " (prime)"
        } else {
            ""
        };
        println!("    spf[{x:>2}] = {}{tag}", small[x]);
    }

    println!("\n  Step-by-step division:");
    for x in [12, 30, 28, 29] {
        factorize_traced(x, &small);
    }

    println!("\n=== Factorize with a bigger table (n = 1_000_000) ===");
    const N: usize = 1_000_000;
    let spf = build_spf(N);
    for x in [1, 12, 97, 360, 999_983, 1_000_000] {
        println!("  {x} = {}", format_factors(&factorize(x, &spf)));
    }

    // Both ideas must agree on which numbers are prime.
    let classic = classic_sieve(N);
    let agree = (2..=N).all(|x| classic[x] == (spf[x] as usize == x));
    println!("\nclassic sieve and SPF agree on primality: {agree}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_small() {
        let s = classic_sieve(30);
        let primes: Vec<usize> = (0..=30).filter(|&i| s[i]).collect();
        assert_eq!(primes, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn edge_cases() {
        assert_eq!(classic_sieve(0), [false]);
        assert_eq!(classic_sieve(1), [false, false]);
        assert_eq!(classic_sieve(2), [false, false, true]);
        assert_eq!(build_spf(0), [0]);
    }

    #[test]
    fn spf_matches_classic() {
        let n = 10_000;
        let c = classic_sieve(n);
        let s = build_spf(n);
        for x in 2..=n {
            assert_eq!(c[x], s[x] as usize == x, "mismatch at {x}");
        }
    }

    #[test]
    fn factorize_roundtrip() {
        let n = 5_000;
        let spf = build_spf(n);
        for x in 1..=n {
            let product: usize = factorize(x, &spf)
                .iter()
                .map(|&(p, e)| (p as usize).pow(e))
                .product();
            assert_eq!(product, x);
        }
    }

    #[test]
    fn factorize_known() {
        let spf = build_spf(1000);
        assert_eq!(factorize(360, &spf), [(2, 3), (3, 2), (5, 1)]);
        assert_eq!(factorize(97, &spf), [(97, 1)]);
        assert!(factorize(1, &spf).is_empty());
    }
}
