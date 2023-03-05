fn is_prime(n: u32) -> bool{
    if n <= 1 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u32;

    for i in 2..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }
    true   
}

pub fn sieve(n: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();

    for x in 0..n {
        if is_prime(x) {
            primes.push(x)
        }
    }

    primes
}