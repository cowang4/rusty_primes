
fn sieve_of_eratosthenes(n: usize) -> Vec<usize>
{
    let mut primes = (0..n)
                        .map(|_| true)
                        .collect::<Vec<bool>>();
    primes[0] = false; // 0 isn't prime
    primes[1] = false; // 1 isn't prime
    for i in 2usize..n {
        let prime = primes[i];
        if prime == false {
            continue;
        }
        ((i*i)..n)
            .step_by(i)
            .map(|i| primes[i] = false)
            .collect::<()>();
    }

    primes
        .iter()
        .enumerate()
        .filter(|(_i,val)| val == &&true)
        .map(|(i,_val)| i)
        .collect::<Vec<usize>>()
}

fn main() {
    let primes = sieve_of_eratosthenes(1000);
    primes
        .iter()
        .map(|i| println!("{}", i))
        .collect::<()>();
}
