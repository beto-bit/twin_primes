use twin_primes::load_primes;

fn main() {
    let primes = load_primes("primes.num").unwrap();

    println!("{:?}", primes.len());
}
